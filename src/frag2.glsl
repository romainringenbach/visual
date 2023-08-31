#version 450

#include <common.glsl>

const float baseSize = 0.1;
float speed = 0.001;

void main() {

    float t = getTime()*speed;

    vec4 dummy = texture(tex,iTexCoord);

    vec3 c;
    float l,z=t;
    vec2 r = vec2(getData(0),getData(1));

    float rAngle = getData(2);
    mat2 rotationMat = mat2(cos(rAngle),sin(rAngle),-sin(rAngle),cos(rAngle));

    for(int i=0;i<3;i++) {
        vec2 uv,p=(gl_FragCoord.xy/r);
        p *= 2.0;
        p -= 1.0;
        p = rotationMat * p;
        p += 1.0;
        p /= 2.0;
        uv=p;
        p-=.5;
        vec2 rr = vec2((r.y - r.x)*sin(rAngle) + r.x,(r.x - r.y)*sin(rAngle) + r.y);
        p.x*=rr.x/rr.y;
        z+=.07;
        l=length(p) * ((getNote(3)/128.0) * 2.0 + 1.0);
        uv+=p/l*(sin(z)+1.)*abs(sin(l*9.-z-z));
        c[i]=.01/length(mod(uv,1.)-.5);
    }

    if(getNote(4) > 0 && iPosition.x < 0){
        c = vec3(0);
    }

    if(getNote(5) > 0 && iPosition.x >= 0){
        c = vec3(0);
    }

    f_color = vec4(c/l,t);
}