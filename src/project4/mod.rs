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
    white_block_time_count : f32,
    black_block_time_count : f32
}

static PROJECT_4_DATA : Lazy<Arc<Mutex<Project4Data>>> = Lazy::new(||{Arc::new(Mutex::new(Project4Data{
    midi_info: MidiInfo::new(),
    zoom_animation: NumberAnimation::new(9.0,3.0,1000.0),
    main_rotation_direction: -1.0,
    sub_rotation_direction: 1.0,
    color_inverted: false,
    white_block_number: 0,
    black_block_number: 0,
    white_block_time_count: 0.0,
    black_block_time_count: 0.0,
}))});

create_project!("Project 4","src/project4/frag.glsl",|_time,_delta_time, notes, velocities, _uniform_register |{

    println!("Note / V : {} / {}", notes[0],velocities[0]);

    let mut project_4_data_l = PROJECT_4_DATA.lock().unwrap();

    project_4_data_l.midi_info.update(notes,velocities);

    if project_4_data_l.midi_info.note_changed(0) && project_4_data_l.midi_info.channel_is_on(0) {
        project_4_data_l.zoom_animation.reset();
    }

});