layout(location = 0) out vec4 f_color;
layout(location = 0) in vec2 iPosition;
layout(location = 1) in vec2 iTexCoord;
/*layout(location = 2) flat in float[22] iData;
layout(location = 2, component = 1) flat in uint[16] iNotes;
layout(location = 2, component = 2) flat in uint[16] iVelocities;
layout(location = 18, component = 1) flat in uint iTime;
layout(location = 19, component = 1) flat in uint iDeltaTime;*/
layout(location = 2) flat in vec4 iData[14];


layout(set = 0, binding = 0) uniform sampler2D tex;

uint getNote(int index){
    if(index >= 0 && index < 16){
        return  uint(iData[index/2][index%2]);
    }
    return 0;
}

uint getVelocity(int index){
    if(index >= 0 && index < 16){
        return  uint(iData[index/2][index%2+2]);
    }
    return 0;
}

float getData(int index){
    if(index >= 0 && index < 16){
        if(index <= 1){
            return iData[8][index+2];
        } else {

            return iData[9+(index-2)/4][index % 4];

        }
    }
    return 0.0;
}

int getTime(){
    return int(iData[8][0]);
}

int getDeltaTime(){
    return int(iData[8][1]);
}