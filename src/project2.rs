use crate::create_project;

create_project!("src/frag2.glsl",|time,delta_time, notes, velocities, data |{
    // do nothing
    let _t = time;
    let _dt = delta_time;
});