use std::sync::Mutex;
use crate::create_project;
use crate::utils;

struct Vec2 {
    x : f32,
    y : f32
}

struct Extension {
    length : f32,
    cumulated_time : f32,
    min_length : f32,
    max_length : f32,
    extension_time : f32,
    extension_type : u8,
    finished : bool,
    position : Vec2
}

impl Extension {
    fn update(&mut self, delta_time : f32){
        if self.max_length != self.length {
            self.cumulated_time += delta_time;

            let v = ((self.max_length - self.min_length)/self.extension_time) * utils::ease_out(self.cumulated_time/self.extension_time);
            self.length = f32::min(self.min_length + v, self.max_length);
            if self.length == self.max_length {
                self.finished = true;
                self.length = 0.0;
            }
        }
    }

    fn reset(&mut self){
        self.cumulated_time = 0.0;
        self.finished = false;
        self.length = self.min_length;
        self.position = Vec2 {
            x : rand::random::<f32>() * 2.0 - 1.0,
            y : rand::random::<f32>() * 2.0 - 1.0,
        }
    }
}

static EXTENSIONS : Lazy<Arc<Mutex<Vec<Extension>>>> = Lazy::new(||{Arc::new(Mutex::new(

    vec![Extension {
        length: 0.0,
        cumulated_time: 0.0,
        min_length: 0.0,
        max_length: 0.5,
        extension_time: 0.9,
        extension_type: 0,
        finished: true,
        position: Vec2 { x: 0.0, y: 0.0 },
    }, Extension {
        length: 0.0,
        cumulated_time: 0.0,
        min_length: 0.0,
        max_length: 0.2,
        extension_time: 0.5,
        extension_type: 4,
        finished: true,
        position: Vec2 { x: 0.0, y: 0.0 },
    }, Extension {
        length: 0.0,
        cumulated_time: 0.0,
        min_length: 0.0,
        max_length: 0.3,
        extension_time: 0.8,
        extension_type: 2,
        finished: true,
        position: Vec2 { x: 0.0, y: 0.0 },
    }, Extension {
        length: 0.0,
        cumulated_time: 0.0,
        min_length: 0.0,
        max_length: 0.1,
        extension_time: 0.5,
        extension_type: 3,
        finished: true,
        position: Vec2 { x: 0.0, y: 0.0 },
    }, Extension {
        length: 0.0,
        cumulated_time: 0.0,
        min_length: 0.0,
        max_length: 0.35,
        extension_time: 0.5,
        extension_type: 1,
        finished: true,
        position: Vec2 { x: 0.0, y: 0.0 },
    },Extension {
        length: 0.0,
        cumulated_time: 0.0,
        min_length: 0.0,
        max_length: 0.35,
        extension_time: 0.5,
        extension_type: 1,
        finished: true,
        position: Vec2 { x: 0.0, y: 0.0 },
    },Extension {
        length: 0.0,
        cumulated_time: 0.0,
        min_length: 0.0,
        max_length: 0.35,
        extension_time: 0.5,
        extension_type: 1,
        finished: true,
        position: Vec2 { x: 0.0, y: 0.0 },
    }]

))});

static CH_OFF: Lazy<Arc<Mutex<Vec<bool>>>> = Lazy::new(||{Arc::new(Mutex::new(vec![true,true,true,true,true]))});
static OLD_CH_OFF: Lazy<Arc<Mutex<Vec<bool>>>> = Lazy::new(||{Arc::new(Mutex::new(vec![true,true,true,true,true]))});

static K: Lazy<Arc<Mutex<u8>>> = Lazy::new(||{Arc::new(Mutex::new(0))});

create_project!("Project 3","src/project3/frag.glsl",|_time,delta_time, _notes, velocities, uniform_register |{

    let mut extension_l = EXTENSIONS.lock().unwrap();
    let mut ch_off_l = CH_OFF.lock().unwrap();
    let mut old_ch_off_l = OLD_CH_OFF.lock().unwrap();

    let mut k_l = K.lock().unwrap();

    if ch_off_l[4] == false && old_ch_off_l[4] == true && !extension_l[4+*k_l as usize].finished {
        *k_l+=1;
        println!("k {}",*k_l);
        if *k_l == 3 {
            *k_l = 0;
        }
    }

    for d in 0..5 {
        old_ch_off_l[d] = ch_off_l[d];
    }

    let mut i = 0;
    for  extension_i in extension_l.iter_mut() {
        let l = i.min(4);



        if velocities[l] > 0 && l < 4 && extension_i.finished {
            extension_i.reset();
            ch_off_l[l] = false;
        }
        if velocities[l] > 0 && l == 4 && i == 4+*k_l as usize && extension_i.finished {
            extension_i.reset();
            ch_off_l[l] = false;
        }

        if velocities[l] == 0 {
            ch_off_l[l] = true;
        }
        i+= 1;
    }

    let delta_time_as_seconds = (delta_time as f32)/1000.0;

    let mut data2 : [[f32;4];7] = [[0.0;4];7];

    i = 0;
    for extension_i in extension_l.iter_mut(){
        if !extension_i.finished {
            extension_i.update(delta_time_as_seconds);
        }

        data2[i][0] = extension_i.length;
        data2[i][1] = extension_i.extension_type.into();
        data2[i][2] = extension_i.position.x;
        data2[i][3] = extension_i.position.y;
        i+=1;
    }

    let uniform_data = fs::Data {
        circleData : data2,
    };

    uniform_register.register_uniform_data(uniform_data);
});