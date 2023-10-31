use std::sync::Mutex;
use midi_msg::BarMarker::Number;
use crate::create_project;
use crate::utils;
use crate::utils::{MidiInfo, NumberAnimation};

/*
    Channels:
    0 : Low Freq Kick, on 1/16 + 9/16
    1 : Percussion with reverb 1 : 5/16
    2 : Percussion with reverb 2 : 13/16
    3 : Percussion with reverb 3 : 2-4/16 on 4/4
    4 : Hat, 6-8 and 14-16 / 16
    5 : Bass  7/16 + 8/16 some time and 10 to 12/16

    Actions
    0 : Zoom in with smooth value and zoom back immediately
    1 : Change main rotation direction
    2 : Change sub rotation direction
    3 : Invert all color permanently
    4 : Replace sub square by white in index order
    5 : Replace sub square by black in index order
 */

struct Project4Data {
    midi_info : MidiInfo,
    zoom_animation : NumberAnimation,
    main_rotation_direction : f32,
    sub_rotation_direction : f32,
    color_inverted: bool,
    white_block_number : u32,
    black_block_number : u32,
    white_block_time_count : u32,
    black_block_time_count : u32
}

static PROJECT_4_DATA : Lazy<Arc<Mutex<Project4Data>>> = Lazy::new(||{Arc::new(Mutex::new(Project4Data{
    midi_info: MidiInfo::new(),
    zoom_animation: NumberAnimation::new(6.0,3.0,500),
    main_rotation_direction: -1.0,
    sub_rotation_direction: 1.0,
    color_inverted: false,
    white_block_number: 0,
    black_block_number: 0,
    white_block_time_count: 0,
    black_block_time_count: 0,
}))});

const BLOCK_TIME_ERASE : u32 = 300;

create_project!("Project 4","src/project4/frag.glsl",|_time,delta_time, notes, velocities, uniform_register |{

    let mut project_4_data_l = PROJECT_4_DATA.lock().unwrap();

    project_4_data_l.midi_info.update(notes,velocities);

    if project_4_data_l.midi_info.velocity_changed(1) && project_4_data_l.midi_info.channel_is_on(1) {
        project_4_data_l.zoom_animation.reset();
    }

    if project_4_data_l.zoom_animation.finished {
        project_4_data_l.zoom_animation.value = project_4_data_l.zoom_animation.from;
    }

    project_4_data_l.zoom_animation.update(delta_time);

    if project_4_data_l.midi_info.velocity_changed(0) && project_4_data_l.midi_info.channel_is_on(0) {
        project_4_data_l.main_rotation_direction *= -1.0;
    }

    if project_4_data_l.midi_info.velocity_changed(2) && project_4_data_l.midi_info.channel_is_on(2) {
        project_4_data_l.sub_rotation_direction *= -1.0;
    }
    if project_4_data_l.midi_info.velocity_changed(3) && project_4_data_l.midi_info.channel_is_on(3) {
        project_4_data_l.color_inverted = !project_4_data_l.color_inverted;
    }

    if project_4_data_l.midi_info.velocity_changed(4) && project_4_data_l.midi_info.channel_is_on(4) {
        project_4_data_l.white_block_number +=1;
        project_4_data_l.white_block_time_count = 0;
    }

    project_4_data_l.white_block_time_count += delta_time;

    if BLOCK_TIME_ERASE <= project_4_data_l.white_block_time_count {
        project_4_data_l.white_block_number = 0;
    }

    if project_4_data_l.midi_info.velocity_changed(5) && project_4_data_l.midi_info.channel_is_on(5) {
        project_4_data_l.black_block_number +=1;
        project_4_data_l.black_block_time_count = 0;
    }

    project_4_data_l.black_block_time_count += delta_time;

    if BLOCK_TIME_ERASE <= project_4_data_l.black_block_time_count {
        project_4_data_l.black_block_number = 0;
    }

    let uniform_data = fs::Data {
        zoomValue : project_4_data_l.zoom_animation.value,
        mainRotationDirection : project_4_data_l.main_rotation_direction,
        subRotationDirection : project_4_data_l.sub_rotation_direction,
        colorInverted : project_4_data_l.color_inverted as u32,
        whiteBlockNumber : project_4_data_l.white_block_number,
        blackBlockNumber : project_4_data_l.black_block_number,
    };
    println!("Values : {} {} {} {} {} {}",project_4_data_l.zoom_animation.value,project_4_data_l.main_rotation_direction,project_4_data_l.sub_rotation_direction,project_4_data_l.color_inverted as u32,project_4_data_l.white_block_number,project_4_data_l.black_block_number);

    uniform_register.register_uniform_data(uniform_data);

});