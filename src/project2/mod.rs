use std::f32::consts::PI;
use std::sync::Mutex;
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

static CH_OFF: Lazy<Arc<Mutex<Vec<bool>>>> = Lazy::new(||{Arc::new(Mutex::new(vec![true,true,true,true,true]))});

static MOD_INDEX: Lazy<Arc<Mutex<i32>>> = Lazy::new(||{Arc::new(Mutex::new(-1))});

create_project!("Project 2","src/project2/frag.glsl",|time,delta_time, _notes, velocities, uniform_register |{
    // do nothing
    let _t = time;
    let _dt = delta_time;

    let mut rotation_l = ROTATION.lock().unwrap();
    let mut ch_off_l = CH_OFF.lock().unwrap();

    let real_velocities = [velocities[0],velocities[1],velocities[2],velocities[3]];

    let mut i = 0;
    for  rotation_i in rotation_l.iter_mut() {
        if real_velocities[i] > 0 && ch_off_l[i] && rotation_i.rotation == rotation_i.wanted_rotation {
            rotation_i.old_rotation = rotation_i.rotation;
            if i%2 == 0 {
                rotation_i.wanted_rotation += rotation_i.rotation_delta;
            } else {
                rotation_i.wanted_rotation -= rotation_i.rotation_delta;
            }
            rotation_i.cumulate_rotation_over_time = 0.0;
            ch_off_l[i] = false;
        } else if real_velocities[i] == 0 {
            ch_off_l[i] = true;
        }

        i+= 1;
    }

    let mut mod_index_l = MOD_INDEX.lock().unwrap();

    if velocities[4] > 0 && ch_off_l[4] == false {
        ch_off_l[4] = true;
        *mod_index_l = (rand::random::<f32>() * 4.0).round() as i32;
    }

    if velocities[4] == 0 && ch_off_l[4] == true {
        ch_off_l[4] = false;
        *mod_index_l = -1;
    }

    let delta_time_as_seconds = (delta_time as f32)/1000.0;

    let mut data2 : [f32;4] = [0.0,0.0,0.0,0.0];
    i = 0;
    for rotation_i in rotation_l.iter_mut(){
        rotation_i.update(delta_time_as_seconds);
        let mut a = 0.0;
        if *mod_index_l >= 0 && i == *mod_index_l as usize {
            a = 180.0;
        }
        data2[i] = rotation_i.rotation+a;
        i+=1;
    }

    let uniform_data = fs::Data {
        rotations : data2,
    };

    uniform_register.register_uniform_data(uniform_data);


});