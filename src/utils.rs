pub fn lerp(start_value : f32, end_value : f32, pct : f32) -> f32{
    return start_value + (end_value - start_value)*pct;
}

pub fn ease_in(t : f32) -> f32{
    return t*t;
}

pub fn flip(x: f32) -> f32{
    return 1.0-x;
}

pub fn ease_out(t: f32) -> f32 {
    return flip(f32::sqrt(flip(t)));
}

pub fn ease_in_out(t: f32) -> f32 {
    return lerp(ease_in(t),ease_out(t),t);
}
