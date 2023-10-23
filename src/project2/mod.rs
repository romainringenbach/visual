use std::f32::consts::PI;
use crate::create_project;
use crate::utils;

struct Rotation {
    rotation : f32,
    wanted_rotation : f32,
    old_rotation : f32,
    cumulate_rotation_over_time : f32,
    rotation_time : f32,
    rotation_delta : f32
}

impl Rotation {
    fn update(&mut self, delta_time : f32){
        if self.wanted_rotation != self.rotation {
            self.cumulate_rotation_over_time += delta_time;

            let v = self.rotation_delta * utils::ease_in_out(self.cumulate_rotation_over_time/self.rotation_time);
            if self.wanted_rotation > self.old_rotation {
                self.rotation = f32::min(self.old_rotation + v, self.wanted_rotation)
            } else if self.wanted_rotation < self.old_rotation {
                self.rotation = f32::max(self.old_rotation - v, self.wanted_rotation)
            }
        }
    }
}


static ROTATION_TIME : f32 = 1.0;
static ROTATION_DELTA : f32 = PI/2.0;

static ROTATION : Lazy<Arc<Mutex<Vec<Rotation>>>> = Lazy::new(||{Arc::new(Mutex::new(

    vec![Rotation {
        rotation: 0.0,
        wanted_rotation: 0.0,
        old_rotation: 0.0,
        cumulate_rotation_over_time: 0.0,
        rotation_time : ROTATION_TIME,
        rotation_delta : ROTATION_DELTA
    },Rotation {
        rotation: 0.0,
        wanted_rotation: 0.0,
        old_rotation: 0.0,
        cumulate_rotation_over_time: 0.0,
        rotation_time : ROTATION_TIME,
        rotation_delta : ROTATION_DELTA
    },Rotation {
        rotation: 0.0,
        wanted_rotation: 0.0,
        old_rotation: 0.0,
        cumulate_rotation_over_time: 0.0,
        rotation_time : ROTATION_TIME,
        rotation_delta : ROTATION_DELTA
    },Rotation {
        rotation: 0.0,
        wanted_rotation: 0.0,
        old_rotation: 0.0,
        cumulate_rotation_over_time: 0.0,
        rotation_time : ROTATION_TIME,
        rotation_delta : ROTATION_DELTA
    }]

))});

static CH_OFF: Lazy<Arc<Mutex<Vec<bool>>>> = Lazy::new(||{Arc::new(Mutex::new(vec![true,true,true,true]))});

create_project!("Project 2","src/project2/frag.glsl",|time,delta_time, _notes, velocities, uniform_register |{
    // do nothing
    let _t = time;
    let _dt = delta_time;

    let mut rotation_l = ROTATION.lock().unwrap();
    let mut ch_off_l = CH_OFF.lock().unwrap();

    let mut i = 0;
    for  rotation_i in rotation_l.iter_mut() {
        if velocities[i] > 0 && ch_off_l[i] && rotation_i.rotation == rotation_i.wanted_rotation {
            rotation_i.old_rotation = rotation_i.rotation;
            if i%2 == 0 {
                rotation_i.wanted_rotation += rotation_i.rotation_delta;
            } else {
                rotation_i.wanted_rotation -= rotation_i.rotation_delta;
            }
            rotation_i.cumulate_rotation_over_time = 0.0;
            ch_off_l[i] = false;
        } else if velocities[i] == 0 {
            ch_off_l[i] = true;
        }

        i+= 1;
    }

    let delta_time_as_seconds = (delta_time as f32)/1000.0;

    let mut data2 : [f32;4] = [0.0,0.0,0.0,0.0];
    i = 0;
    for rotation_i in rotation_l.iter_mut(){
        rotation_i.update(delta_time_as_seconds);
        data2[i] = rotation_i.rotation;
        i+=1;
    }

    let uniform_data = fs::Data {
        rotations : data2,
    };

    uniform_register.register_uniform_data(uniform_data);


});