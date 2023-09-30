use crate::create_project;

create_project!("Project 1", "src/project1/frag.glsl",|time,delta_time, _notes, _velocities, _data, _uniform_register |{
    // do nothing
    let _t = time;
    let _dt = delta_time;
});