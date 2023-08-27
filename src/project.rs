#![macro_use]
use vulkano::device::Device;
use std::sync::Arc;
use vulkano::shader::{ShaderCreationError, ShaderModule};

pub struct Project
{
    pub frag_loader : fn(Arc<Device>) -> Result<Arc<ShaderModule>,ShaderCreationError>,
    pub update : fn(u32,u32) // time, deltaTime, midi data, push constant available data array
}

#[macro_export]
macro_rules! create_project {
    ($frag:stmt, $update:expr) => {

        pub mod fs {
            vulkano_shaders::shader! {
                ty: "fragment",
                include : ["src/"],
                path: $frag
            }
        }

        use crate::project::Project;
        use once_cell::sync::Lazy;

        pub static PROJECT : Lazy<Box<Project>> = Lazy::new(||{Box::new(Project {
            frag_loader : fs::load,
            update : $update
        })});
    };
}