#version 450

#include <common.glsl>

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

    return _st;
}

void main() {

    vec4 dummy = texture(tex,iTexCoord);
    float time = common_data.time;

    vec2 st = gl_FragCoord.xy / common_data.screenSize;

    float speed = 0.0001;

    st.y *= float(common_data.screenSize.y) / float(common_data.screenSize.x);

    st = rotate2D(st,-PI*time*speed);
    st = tile(st,vec2(3.0,3.0));

    st = rotate2D(st,PI*time*speed);
    st = subTile(st);
    st = rotate2D(st,-PI*time*speed);
    st = rotate2D(st,PI*time*speed);


    f_color = vec4(vec3(step(st.x,st.y)),1);
}