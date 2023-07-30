#version 450

layout(location = 0) out vec4 f_color;
layout(location = 0) in vec2 iPosition;
layout(location = 1) in vec2 iTexCoord;

layout(set = 0, binding = 0) uniform sampler2D tex;

// Our variable inputs as push constants
layout(push_constant) uniform PushConstants {
    uint note[8];
    uint velocity[8];
    uint time;
} push_constants;

/*
    channels :
        0 : "high rythm" -> shader blinking
        1 : "bass" -> baseSize * 2
        2 : "verb_beep" -> baseSize * 0.5
        3 : "high_tone" -> baseSize * 0.25
*/

const float baseSize = 0.1;
const float speed = 0.00001;

float rand(vec2 co){
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}


void main() {

    float ya = 0;
    if(iPosition.y > 0.0){
        ya = 0.5;
    }

    float sizeFactor = 1.0;
    if(push_constants.note[1] > 0){
        sizeFactor = 2.0;
    } else if(push_constants.note[2] > 0){
        sizeFactor = 0.5;
    } else if(push_constants.note[3] > 0){
        sizeFactor = 0.25;
    }

    float c = round(texture(tex,vec2(iTexCoord.x * baseSize * sizeFactor,push_constants.time * speed + ya)).r);

    if(push_constants.note[0] > 0){
        c = 0;
    }

    f_color = vec4(c,c,c,1);
}