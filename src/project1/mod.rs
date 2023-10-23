use crate::create_project;

static COMPUTED_TIME: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{Arc::new(Mutex::new(0.0))});
static SIZE_FACTOR: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{Arc::new(Mutex::new(1.0))});

const DEFAULT_SPEED : f32 = 0.00001;

static SPEED: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{Arc::new(Mutex::new(DEFAULT_SPEED))});

static OLD_VELOCITIES: Lazy<Arc<Mutex<Vec<u32>>>> = Lazy::new(||{Arc::new(Mutex::new(vec![0,0,0,0,0,0]))});

create_project!("Project 1", "src/project1/frag.glsl",|_time,delta_time, _notes, velocities, uniform_register |{
    let mut old_velocities = OLD_VELOCITIES.lock().unwrap();

    let mut time_l = COMPUTED_TIME.lock().unwrap();
    let mut size_factor_l = SIZE_FACTOR.lock().unwrap();
    let mut speed_l = SPEED.lock().unwrap();


    if velocities[0] > 0 && old_velocities[0] == 0 {
        *size_factor_l = 4.0;
    //} else if velocities[3] > 0 && old_velocities[3] == 0 {
    //    *size_factor_l = 0.5;
    } else if velocities[4] > 0 && old_velocities[4] == 0 {
        *size_factor_l = 0.1;
    } else if velocities[0] == 0 && velocities[4] == 0 {
        *size_factor_l = 1.0;
    }
    
    let mut color_factor : f32 = 1.0;
    
    if velocities[2] > 0 && old_velocities[2] == 0 {
        color_factor = 0.0;
    }

    if velocities[1] > 0 {
        *speed_l = 0.0001;
    } else if velocities[5] > 0 && old_velocities[5] == 0 {
        *speed_l = 0.0;
    } else if velocities[0] > 0 && old_velocities[0] == 0{
        *speed_l = DEFAULT_SPEED;
    }

    *time_l += delta_time as f32 * *speed_l;
    *time_l = *time_l % 1.0;

    for d in 0..5 {
        old_velocities[d] = velocities[d];
    }

    let uniform_data = fs::Data {
        time : *time_l,
        sizeFactor : *size_factor_l,
        colorFactor : color_factor,
    };

    uniform_register.register_uniform_data(uniform_data);
});