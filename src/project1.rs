use crate::create_project;

create_project!("src/frag.glsl",|time,delta_time, notes, velocities, data, _uniform_register |{
    // do nothing
    let _t = time;
    let _dt = delta_time;
    data[0] = notes[0].into();
    data[1] = velocities[0].into();
    data[2] = 0.5;
});