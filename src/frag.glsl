#version 450

layout(location = 0) out vec4 f_color;
layout(location = 0) in vec2 iPosition;

// Our variable inputs as push constants
layout(push_constant) uniform PushConstants {
    uint note[16];
    uint velocity[16];
} push_constants;

void main() {

    int c = 0;
    if(iPosition.x < 0){
        if(iPosition.y < 0){
            c = 0;
        } else {
            c = 2;
        }
    } else {
        if(iPosition.y <0 0){
            c = 1;
        } else {
            c = 3;
        }
    }

    f_color = vec4(push_constants.note[c]/127.0,push_constants.velocity[c]/127.0,c/3,1);
}