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


pub struct NumberAnimation {
    pub value: f32,
    pub from: f32,
    pub to: f32,
    pub duration : u32,
    cumulated_time : u32,
    pub finished : bool,
}

impl NumberAnimation {

    pub fn new(from: f32, to: f32, duration : u32) -> Self {
        assert_ne!(from, to, "From and to have to be different from each other");
        assert_ne!(duration,0, "Duration have to be superior to 0");
        NumberAnimation {
            value: from,
            from,
            to,
            duration,
            cumulated_time: 0,
            finished: true,
        }
    }
    pub fn update(&mut self, delta_time : u32){
        if !self.finished {
            self.cumulated_time += delta_time;

            let v = (self.to - self.from) * ease_in_out((self.cumulated_time as f32)/(self.duration as f32));
            if self.from < self.to {
                self.value = f32::min(self.from + v, self.to);
            } else {
                self.value = f32::max(self.from + v, self.to);
            }

            if self.value == self.to {
                self.finished = true;
            }
        }
    }

    pub fn reset(&mut self){
        self.cumulated_time = 0;
        self.finished = false;
        self.value = self.from;
    }
}

pub struct MidiInfo {
    notes : [u32;16],
    velocities : [u32;16],
    old_notes : [u32;16],
    old_velocities : [u32;16],
}

pub const DEFAULT_CHANNEL_VALUES : [u32;16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

impl MidiInfo {
    pub fn new() -> Self {
        MidiInfo {
            notes : DEFAULT_CHANNEL_VALUES,
            velocities : DEFAULT_CHANNEL_VALUES,
            old_notes : DEFAULT_CHANNEL_VALUES,
            old_velocities : DEFAULT_CHANNEL_VALUES
        }
    }

    pub fn update(&mut self, notes : [u32;16], velocities : [u32;16] ) {
        self.old_notes = self.notes;
        self.old_velocities = self.velocities;
        self.notes = notes;
        self.velocities = velocities;
    }

    pub fn note_changed(&self, ch : usize) -> bool {
        assert!(ch >= 0 && ch < 16, "Channel is outside of range [0..16], value : {}", ch);
        return self.notes[ch] != self.old_notes[ch];
    }

    pub fn velocity_changed(&self, ch : usize) -> bool {
        assert!(ch >= 0 && ch < 16, "Channel is outside of range [0..16], value : {}", ch);
        return self.velocities[ch] != self.old_velocities[ch];
    }

    pub fn channel_is_on(&self, ch : usize) -> bool {
        assert!(ch >= 0 && ch < 16, "Channel is outside of range [0..16], value : {}", ch);
        return self.notes[ch] > 0 && self.velocities[ch] > 0;
    }
}

