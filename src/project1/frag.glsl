#version 450

#include <common.glsl>

/*
    channels :

        0 : "bass" -> baseSize * 2
        1 : "beep beep" -> accelerating speed
        2 : "impulse" -> shader blinking
        3 : "high_tone" -> baseSize * 0.25
        4 : "high_tone2" -> baseSize * 0.10
        5 : "verb_beep" -> slowing speed


*/

layout(set = 2, binding = 0) buffer Data {
    float time;
    float sizeFactor;
    float colorFactor;
} uniforms;

/* UData :
    0 : computedTime from speed
   FData :
    0 : baseSize
    1 : colorFactor
    */

void main() {

    uint iTime = common_data.time;

    float ya = 0;
    if(iPosition.y > 0.0){
        ya = 0.5;
    }

    float c = round(texture(tex,vec2(iTexCoord.x * 0.1 / uniforms.sizeFactor, uniforms.time + ya)).r) * uniforms.colorFactor;

    f_color = vec4(c,c,c,1);
}