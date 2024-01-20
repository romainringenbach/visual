#version 450

#include <common.glsl>

layout(set = 2, binding = 0) buffer Data {
    uint colorInverted;
    float density;
} uniforms;

float random (vec2 st) {
    return fract(sin(dot(st.xy,
    vec2(12.9898,78.233)))*
    43758.5453123);
}

void main() {

    vec4 dummy = texture(tex,iTexCoord);
    float time = common_data.time;

    vec2 st = gl_FragCoord.xy / common_data.screenSize;
    st.y *= float(common_data.screenSize.y) / float(common_data.screenSize.x);
    st *= 200;
    st.x += time*0.1*random(floor(vec2(0,st.y+1)));
    vec2 ipos = floor(st);
    vec2 fpos = fract(st);
    // Assign a random value based on the integer coord
    float v = random( vec2(ipos.x,ipos.y) );
    if(v < (1.0-uniforms.density)){
        v = 0;
    } else {
        v = 1;
    }
    if(int(ipos.y)%2 == 1){
        v = 0;
    }

    if(uniforms.colorInverted > 0){
        if(v == 1.0){
            v = 0.0;
        } else {
            v = 1.0;
        }
    }
    vec3 color = vec3(v);


    f_color = vec4(color,1);
}