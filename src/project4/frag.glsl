#version 450

#include <common.glsl>

layout(set = 2, binding = 0) buffer Data {
    float zoomValue;
    float mainRotationDirection;
    float subRotationDirection;
    uint colorInverted;
    uint whiteBlockNumber;
    uint blackBlockNumber;
} uniforms;

vec2 tile(vec2 _st,vec2 _zoom){
    _st *= _zoom;
    return fract(_st);
}

vec2 subTile(vec2 _st){
    //  Scale the coordinate system by 2x2
    _st *= 2.0;

    //  Give each cell an index number
    //  according to its position
    float index = 0.0;
    index += step(1., mod(_st.x,2.0));
    index += step(1., mod(_st.y,2.0))*2.0;

    //      |
    //  2   |   3
    //      |
    //--------------
    //      |
    //  0   |   1
    //      |

    // Make each cell between 0.0 - 1.0
    _st = fract(_st);

    // Rotate each cell according to the index
    if(index == 1.0){
        //  Rotate cell 1 by 90 degrees
        _st = rotate2D(_st,PI*0.5);
    } else if(index == 2.0){
        //  Rotate cell 2 by -90 degrees
        _st = rotate2D(_st,PI*-0.5);
    } else if(index == 3.0){
        //  Rotate cell 3 by 180 degrees
        _st = rotate2D(_st,PI);
    }
    if(index == 0.0 && uniforms.whiteBlockNumber >= 1){
        _st = vec2(1.0,1.0);
    } else if(index == 1.0 && uniforms.whiteBlockNumber >= 2){
        _st = vec2(1.0,1.0);
    } else if(index == 2.0 && uniforms.whiteBlockNumber >= 3){
        _st = vec2(1.0,1.0);
    } else if(index == 3.0 && uniforms.whiteBlockNumber >= 4){
        _st = vec2(1.0,1.0);
    } else if(index == 0.0 && uniforms.blackBlockNumber >= 1){
        _st = vec2(0.0,0.0);
    } else if(index == 1.0 && uniforms.blackBlockNumber >= 2){
        _st = vec2(0.0,0.0);
    } else if(index == 2.0 && uniforms.blackBlockNumber >= 3){
        _st = vec2(0.0,0.0);
    } else if(index == 3.0 && uniforms.blackBlockNumber >= 4){
        _st = vec2(0.0,0.0);
    }

    return _st;
}

void main() {

    vec4 dummy = texture(tex,iTexCoord);
    float time = common_data.time;

    vec2 st = gl_FragCoord.xy / common_data.screenSize;

    float speed = 0.0001;

    st.y *= float(common_data.screenSize.y) / float(common_data.screenSize.x);

    st = rotate2D(st,uniforms.mainRotationDirection*PI*time*speed);
    st = tile(st,vec2(uniforms.zoomValue));

    st = rotate2D(st,uniforms.subRotationDirection*PI*time*speed);
    st = subTile(st);
    st = rotate2D(st,-uniforms.subRotationDirection*PI*time*speed);
    st = rotate2D(st,-uniforms.mainRotationDirection*PI*time*speed);

    float c = step(st.x,st.y);
    if(uniforms.colorInverted > 0){
        if(c == 1.0){
            c = 0.0;
        } else {
            c = 1.0;
        }
    }

    f_color = vec4(vec3(c),1);
}