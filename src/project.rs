#![macro_use]
use vulkano::device::Device;
use std::sync::{Arc};
use vulkano::shader::{ShaderCreationError, ShaderModule};
use crate::uniform_register::UniformRegister;

pub struct Project
{
    pub name: String,
    pub frag_loader : fn(Arc<Device>) -> Result<Arc<ShaderModule>,ShaderCreationError>,
    pub update : fn(u32,u32,[u8;16],[u8;16],& mut [f32;22], & mut UniformRegister) // time, deltaTime, midi data, push constant available data array
}

#[macro_export]
macro_rules! create_project {
    ($name:expr, $frag:stmt, $update:expr) => {

        pub mod fs {
            vulkano_shaders::shader! {
                ty: "fragment",
                include : ["src/"],
                path: $frag
            }
        }

        use crate::project::Project;
        use once_cell::sync::Lazy;
        use std::sync::{Arc,Mutex,RwLock};

        pub static PROJECT : Lazy<Arc<RwLock<Project>>> = Lazy::new(||Arc::new(RwLock::new(Project {
            name: String::from($name),
            frag_loader : fs::load,
            update : $update
        })));
    };
}