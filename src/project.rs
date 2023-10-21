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
    time: Padded<u32,12>,
    deltaTime: Padded<u32,12>,
    screenSize: Padded<[u32;2],8>,
    midiNotes: [Padded<u32,12>;16],
    midiVelocities: [Padded<u32,12>;16]
}

impl Project {
    pub fn fillCommonData(&self,time:u32,delta_time:u32,screen_width:u32,screen_height:u32,midi_notes:[u32;16],midi_velocities: [u32;16], uniform_register: & mut UniformRegister){

        let mut midiNotes: [Padded<u32,12>;16] = [Padded(0),Padded(0),Padded(0),Padded(0),Padded(0),Padded(0),Padded(0),Padded(0),Padded(0),Padded(0),Padded(0),Padded(0),Padded(0),Padded(0),Padded(0),Padded(0)];
        let mut midiVelocities: [Padded<u32,12>;16] = midiNotes;

        let mut i = 0;
        for midi_note in midi_notes {
            midiNotes[i] = Padded(midi_note);
            i+=1;
        }

        i = 0;
        for midi_velocity in midi_velocities {
            midiVelocities[i] = Padded(midi_velocity);
            i+=1;
        }

        let common_data = CommonData {
            time: Padded(time),
            deltaTime: Padded(delta_time),
            screenSize: Padded([screen_width,screen_height]),
            midiNotes,
            midiVelocities,
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