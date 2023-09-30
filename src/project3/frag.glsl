#version 450

#include <common.glsl>

layout(set = 1, binding = 0) uniform Data {
    vec4 circleData[7];
} uniforms;

const vec2 right = vec2(1,0);

bool isWhite(float length2, int type, vec2 center, vec2 screenSize, float time){

    vec2 realPos = iPosition;
    realPos.y *= screenSize.y/ screenSize.x;

    if(length(realPos-center) <= length2){

        if(type == 0){

            return true;

        } else if(type == 1){

            if(length(realPos-center) >= length2-0.01){
                return true;
            }

        } else if(type == 2){
            float angle = acos(dot(normalize(realPos-center),normalize(right)));
            if(realPos.y >= 0){
                angle = radians(360) - angle;
            }
            float angleP = angle / radians(360);
            float c = round(texture(tex,vec2(angleP,time)).r);

            return c == 1.0;
        }
    }

    return false;
}


void main() {

    uint iTime = getTime();

    float c = 0.0;
    vec2 r = vec2(getData(0),getData(1));

    for (int i = 0; i < 7 ; i++){
        float note = getNote(i);

        vec4 dd = uniforms.circleData[i];

        float length = dd[0];
        int type = int(dd[1]);

        vec2 center = vec2(dd[2],dd[3]);
        if(isWhite(length,type,center,r,iTime)){
            c= 1.0;
        }
    }

    f_color = vec4(c,c,c,1);
}