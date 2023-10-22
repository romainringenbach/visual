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

float rand(vec2 co){
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
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