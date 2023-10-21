#version 450

#include <common.glsl>

/*
    channels :
        2 : "impulse" -> shader blinking
        0 : "bass" -> baseSize * 2
        5 : "verb_beep" -> baseSize * 0.5
        3 : "high_tone" -> baseSize * 0.25
        4 : "high_tone2" -> baseSize * 0.10
        1 : "beep beep" -> baseSize * 0.75
*/

const float baseSize = 0.1;
const float sizeF[6] = {2.,1.0,1.0,0.25,0.1,1.0};

void main() {

    uint iTime = common_data.time;

    float ya = 0;
    if(iPosition.y > 0.0){
        ya = 0.5;
    }

    float sizeFactor = 1.0;
    float speed = 0.00001;
    float f = 1.0;

    for(int i =0;i < 6; i++){
        if(common_data.midiVelocities[i] > 0){
            if(i == 2){
                f = 0.0;
            } else if(i == 5){
                speed = 0.00005;
            } else if(i == 1) {
                speed = 0.000005;
            } else {
                sizeFactor = sizeF[i];
            }
        }
    }
    float c = round(texture(tex,vec2(iTexCoord.x * baseSize / sizeFactor,iTime * speed + ya)).r) * f;

    //f_color = vec4(c,c,c,1);

    f_color = checkMidi();
}