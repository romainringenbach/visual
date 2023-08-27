use crate::create_project;

create_project!("src/frag.glsl",|time,delta_time |{
    // do nothing
    let _t = time;
    let _dt = delta_time;
});