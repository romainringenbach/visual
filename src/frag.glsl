#version 450

#include <common.glsl>

/*
    channels :
        0 : "impulse" -> shader blinking
        1 : "bass" -> baseSize * 2
        2 : "verb_beep" -> baseSize * 0.5
        3 : "high_tone" -> baseSize * 0.25
        3 : "high_tone2" -> baseSize * 0.25
*/

const float baseSize = 0.1;
float speed = 0.00001;

float rand(vec2 co){
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}


void main() {

    float ya = 0;
    if(iPosition.y > 0.0){
        ya = 0.5;
    }

    float sizeFactor = 1.0;
    if(iNotes[1] > 0){
        sizeFactor = 4.0;
    }

    if(iNotes[2] > 0){
        speed = 0.00005;
    }

    if(iNotes[3] > 0){
        sizeFactor = 0.25;
    }

    if(iNotes[4] > 0){
        sizeFactor = 0.1;
    }

    float c = round(texture(tex,vec2(iTexCoord.x * baseSize / sizeFactor,iTime * speed + ya)).r);

    if(iNotes[0] > 0){
        c = 0;
    }

    f_color = vec4(c,c,c,1);
}