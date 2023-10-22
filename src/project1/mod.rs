use vulkano_macros::BufferContents;
use crate::create_project;

static COMPUTED_TIME: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{Arc::new(Mutex::new(0.0))});
static SIZE_FACTOR: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{Arc::new(Mutex::new(1.0))});

const SPEED : f32 = 0.00001;

static OLD_VELOCITIES: Lazy<Arc<Mutex<Vec<u32>>>> = Lazy::new(||{Arc::new(Mutex::new(vec![0,0,0,0,0,0]))});

create_project!("Project 1", "src/project1/frag.glsl",|_time,delta_time, _notes, velocities, uniform_register |{
    let mut old_velocities = OLD_VELOCITIES.lock().unwrap();

    let mut time_l = COMPUTED_TIME.lock().unwrap();
    let mut size_factor_l = SIZE_FACTOR.lock().unwrap();

    let mut speed = SPEED;

    if velocities[1] > 0 {
        speed = 0.0001;
    } else if velocities[5] > 0 {
        speed = 0.00001;
    }

    if velocities[0] > 0 && old_velocities[0] == 0 {
        *size_factor_l = 4.0;
    } else if velocities[3] > 0 && old_velocities[3] == 0 {
        *size_factor_l = 0.5;
    } else if velocities[4] > 0 && old_velocities[4] == 0 {
        *size_factor_l = 0.1;
    } else if velocities[0] == 0 && velocities[3] == 0 && velocities[4] == 0 {
        *size_factor_l = 1.0;
    }
    
    let mut color_factor : f32 = 1.0;
    
    if velocities[2] > 0 && old_velocities[2] == 0 {
        color_factor = 0.0;
    } 

    *time_l += delta_time as f32 * speed;
    *time_l = *time_l % 1.0;

    for d in 0..5 {
        old_velocities[d] = velocities[d];
    }

    //println!("Debug : {} {} {} {}",*time_l,time_l.round() as u32,base_size,color_factor);

    let uniform_data = fs::Data {
        time : *time_l,
        sizeFactor : *size_factor_l,
        colorFactor : color_factor,
    };

    uniform_register.register_uniform_data(uniform_data);
});