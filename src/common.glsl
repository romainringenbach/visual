layout(location = 0) out vec4 f_color;
layout(location = 0) in vec2 iPosition;
layout(location = 1) in vec2 iTexCoord;

layout(set = 0, binding = 0) uniform sampler2D tex;
layout(set = 1, binding = 0) buffer CommonData {
    uint time;
    uint deltaTime;
    uvec2 screenSize;
    uint midiNotes[16];
    uint midiVelocities[16];
} common_data;

#define PI 3.14159265358979323846

float rand(vec2 co){
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}

// rotate element on  0..1:0..1 coordonates
vec2 rotate2D (vec2 _st, float _angle) {
    _st -= 0.5;
    _st =  mat2(cos(_angle),-sin(_angle),
    sin(_angle),cos(_angle)) * _st;
    _st += 0.5;
    return _st;
}

vec4 checkMidi(){

    uint channel_index = uint(floor(iTexCoord.x*16.0));
    channel_index = min(channel_index,15);

    if(iTexCoord.y < 0.5){
        uint note = common_data.midiNotes[channel_index];
        float note_to_color = float(note)/128.0;
        return vec4(note_to_color,mod(channel_index,2.0) == 0 ? note_to_color : 0.1,mod(channel_index,2.0) == 0 ? 0.1 : note_to_color,1.0);
    } else {
        uint velocity = common_data.midiVelocities[channel_index];
        float velocity_to_color = float(velocity)/128.0;
        return vec4(velocity_to_color,mod(channel_index,2.0) == 0 ? 0.2 : velocity_to_color,mod(channel_index,2.0) == 0 ? velocity_to_color : 0.2,1.0);
    }
}