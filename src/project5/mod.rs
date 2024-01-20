use std::sync::Mutex;
use crate::create_project;
use crate::utils::MidiInfo;

/*

 */

struct Project5Data {
    midi_info: MidiInfo,
    color_inverted: bool,
    density: f32,
    cumulated_trigger_count: u32,
}

const MAX_TRIGGER_NUMER : u32 = 200;

static PROJECT_5_DATA : Lazy<Arc<Mutex<crate::project5::Project5Data>>> = Lazy::new(||{Arc::new(Mutex::new(crate::project5::Project5Data {
    midi_info: MidiInfo::new(),
    color_inverted: false,
    density: 0.0,
    cumulated_trigger_count: 0,
}))});

create_project!("Project 5","src/project5/frag.glsl",|_time,_delta_time, notes, velocities, uniform_register |{
    let mut project_5_data_l = PROJECT_5_DATA.lock().unwrap();
    project_5_data_l.midi_info.update(notes,velocities);

    println!("test pr5 midi : {} {} {} {}",notes[0],velocities[0],project_5_data_l.midi_info.channel_off_changed(0),project_5_data_l.midi_info.channel_is_on(0));

    if project_5_data_l.midi_info.channel_off_changed(0) && project_5_data_l.midi_info.channel_is_on(0) {
        project_5_data_l.cumulated_trigger_count+=1;
        project_5_data_l.color_inverted = !project_5_data_l.color_inverted;
    }
    if project_5_data_l.midi_info.channel_off_changed(1) && project_5_data_l.midi_info.channel_is_on(1) {
        project_5_data_l.cumulated_trigger_count+=1;
    }
    if project_5_data_l.midi_info.channel_off_changed(2) && project_5_data_l.midi_info.channel_is_on(2) {
        project_5_data_l.cumulated_trigger_count+=1;
    }
    if project_5_data_l.midi_info.channel_off_changed(4) && project_5_data_l.midi_info.channel_is_on(4) {
        project_5_data_l.cumulated_trigger_count+=1;
    }
    if project_5_data_l.midi_info.channel_off_changed(5) && project_5_data_l.midi_info.channel_is_on(5) {
        project_5_data_l.cumulated_trigger_count+=1;
    }

    if project_5_data_l.midi_info.channel_off_changed(3) && project_5_data_l.midi_info.channel_is_on(3) {
        project_5_data_l.cumulated_trigger_count = 0;
    }

    project_5_data_l.density = project_5_data_l.cumulated_trigger_count as f32 / MAX_TRIGGER_NUMER as f32;

    let uniform_data = fs::Data {
        colorInverted : project_5_data_l.color_inverted as u32,
        density : project_5_data_l.density,
    };
    uniform_register.register_uniform_data(uniform_data);

});