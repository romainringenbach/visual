#version 450

layout(push_constant) uniform PushConstants {
    uint time;
    uint deltaTime;
    uint midiData[8];
    float data[22];
} push_constants;

layout(location = 0) in vec2 position;
layout(location = 0) out vec2 iPosition;
layout(location = 1) out vec2 iTexCoord;
/*layout(location = 1) out float[22] iData;
layout(location = 1, component = 1) out uint[16] iNotes;
layout(location = 1, component = 2) out uint[16] iVelocities;
layout(location = 17, component = 1) out uint iTime;
layout(location = 18, component = 1) out uint iDeltaTime;*/
layout(location = 2) out vec4 iData[14];

const uvec4 decoder = uvec4(1,255,65025,16581375);
const vec4 decoderF = vec4(1.,255.,65025.,16581375.);

uvec4 decode(uint value){
    uvec4 dec = uvec4(0,0,0,0);

    float v = float(value);

    dec[3] = int(floor( v / decoderF[3]));
    dec[2] = int(floor( float(value - dec[3]) / decoderF[2]  ));
    dec[1] = int(floor( float(value - dec[3] - dec[2]) / decoderF[1]  ));
    dec[0] = value - dec[3] - dec[2] - dec[1];

    return dec;
}

void main() {

    iPosition = position;
    iTexCoord = (position+vec2(1.0))/vec2(2.0);

    //uint[16] notes;
    //uint[16] velocities;

    for (int i = 0 ; i < 8 ; i++){
        uvec4 d = decode(push_constants.midiData[i]);
        /*notes[i*2] = d[0];
        velocities[i*2] = d[1];
        notes[i*2+1] = d[2];
        velocities[i*2+1] = d[3];*/

        iData[i] = vec4(d);
    }

    /*iTime = push_constants.time;
    iDeltaTime = push_constants.deltaTime;
    iNotes = notes;
    iVelocities = velocities;*/

    iData[8] = vec4(push_constants.time,push_constants.deltaTime,push_constants.data[0],push_constants.data[1]);

    for(int i = 9 ; i < 14 ; i++){
        int ii = 2+(i-9)*4;
        iData[i] = vec4(push_constants.data[ii],push_constants.data[ii+1],push_constants.data[ii+2],push_constants.data[ii+3]);
    }

    gl_Position = vec4(position, 0.0, 1.0);
}