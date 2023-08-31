pub fn lerp(start_value : f32, end_value : f32, pct : f32) -> f32{
    return (start_value + (end_value - start_value)*pct);
}

pub fn easeIn(t : f32) -> f32{
    return t*t;
}

pub fn flip(x: f32) -> f32{
    return 1.0-x;
}

pub fn easeOut(t: f32) -> f32 {
    return flip(f32::sqrt(flip(t)));
}

pub fn easeInOut(t: f32) -> f32 {
    return lerp(easeIn(t),easeOut(t),t);
}