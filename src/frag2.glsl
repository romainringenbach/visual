#version 450

#include <common.glsl>

const float baseSize = 0.1;
float speed = 0.001;

float rand(vec2 co){
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}


void main() {

    float t = getTime()*speed;

    vec4 dummy = texture(tex,iTexCoord);

    vec3 c;
    float l,z=t;
    vec2 r = vec2(getData(0),getData(1));
    for(int i=0;i<3;i++) {
        vec2 uv,p=gl_FragCoord.xy/r;
        uv=p;
        p-=.5;
        p.x*=r.x/r.y;
        z+=.07;
        l=length(p);
        uv+=p/l*(sin(z)+1.)*abs(sin(l*9.-z-z));
        c[i]=.01/length(mod(uv,1.)-.5);
    }

    f_color = vec4(c/l,t);
}