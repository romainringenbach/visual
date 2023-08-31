use std::cmp::{max, min};
use std::f32::consts::PI;
use std::sync::mpsc::channel;
use crate::create_project;
use crate::utils;
static wanted_rotation: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{Arc::new(Mutex::new(0.0))});
static old_rotation: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{Arc::new(Mutex::new(0.0))});

static cumulate_rotation_time: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{Arc::new(Mutex::new(0.0))});
static rotation_time : f32 = 2.0;
static rotation_delta : f32 = PI/2.0;

static rotation: Lazy<Arc<Mutex<f32>>> = Lazy::new(||{Arc::new(Mutex::new(0.0))});
static ch1_off: Lazy<Arc<Mutex<bool>>> = Lazy::new(||{Arc::new(Mutex::new(true))});
static ch2_off: Lazy<Arc<Mutex<bool>>> = Lazy::new(||{Arc::new(Mutex::new(true))});

create_project!("src/frag2.glsl",|time,delta_time, notes, velocities, data |{
    // do nothing
    let _t = time;
    let _dt = delta_time;

    let mut wanted_rotation_l = wanted_rotation.lock().unwrap();
    let mut cumulate_rotation_time_l = cumulate_rotation_time.lock().unwrap();
    let mut old_rotation_l = old_rotation.lock().unwrap();
    let mut rotation_l = rotation.lock().unwrap();
    let mut ch1_off_l = ch1_off.lock().unwrap();
    let mut ch2_off_l = ch2_off.lock().unwrap();

    if(notes[0] > 0 && *ch1_off_l ){
        if(*wanted_rotation_l != *rotation_l){
            *rotation_l = *old_rotation_l;
        } else {
            *old_rotation_l = *rotation_l;
            *wanted_rotation_l += rotation_delta;
        }
        *cumulate_rotation_time_l = 0.0;
        *ch1_off_l = false;
    } else if(notes[0] == 0){
        *ch1_off_l = true;
    }

    if(notes[1] > 0 && *ch2_off_l ){
        if(*wanted_rotation_l != *rotation_l){
            *rotation_l = *old_rotation_l;
        } else {
            *old_rotation_l = *rotation_l;
            *wanted_rotation_l -= rotation_delta;
        }
        *cumulate_rotation_time_l = 0.0;
        *ch2_off_l = false;
    } else if(notes[0] == 0){
        *ch2_off_l = true;
    }

    let dtAsS = (delta_time as f32)/1000.0;

    if(*wanted_rotation_l != *rotation_l){
        *cumulate_rotation_time_l += dtAsS;

        let v = rotation_delta * utils::easeInOut(*cumulate_rotation_time_l/rotation_time);
        if(*wanted_rotation_l > *rotation_l){
            *rotation_l = f32::min(*rotation_l + v, *wanted_rotation_l)
        } else if(*wanted_rotation_l < *rotation_l){
            *rotation_l = f32::max(*rotation_l - v, *wanted_rotation_l)
        }
    }

    data[2] = *rotation_l;

    //println!("Debug: {0} {1} {2} {3} {4} {5}", notes[0], notes[1], data[2], delta_time, *wanted_rotation_l, *cumulate_rotation_time_l );
});