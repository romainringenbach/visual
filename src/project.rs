#![macro_use]
use vulkano::device::Device;
use std::sync::{Arc};
use vulkano::buffer::BufferContents;
use vulkano::padded::Padded;
use vulkano::shader::{ShaderCreationError, ShaderModule};
use crate::uniform_register::UniformRegister;

pub struct Project
{
    pub name: String,
    pub frag_loader : fn(Arc<Device>) -> Result<Arc<ShaderModule>,ShaderCreationError>,
    pub update : fn(u32,u32,[u32;16],[u32;16], & mut UniformRegister) // time, deltaTime, midi data, push constant available data array
}

#[derive(BufferContents)]
#[repr(C)]
struct CommonData {
    time: u32,
    deltaTime: u32,
    screenSize: [u32;2],
    midiNotes: [u32;16],
    midiVelocities: [u32;16]
}

impl Project {
    pub fn fillCommonData(&self,time:u32,delta_time:u32,screen_width:u32,screen_height:u32,midi_notes:[u32;16],midi_velocities: [u32;16], uniform_register: & mut UniformRegister){

        //println!("Data {} {} {} {}",time,delta_time,screen_width,screen_height);
        //println!("Notes : {:?}",midi_notes);
        //println!("Velocities : {:?}",midi_velocities);

        let common_data = CommonData {
            time,
            deltaTime: delta_time,
            screenSize: [screen_width,screen_height],
            midiNotes : midi_notes,
            midiVelocities : midi_velocities,
        };

        uniform_register.register_uniform_data(common_data);
    }
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