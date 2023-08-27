#version 450

layout(location = 0) in vec2 position;
layout(location = 0) out vec2 iPosition;
layout(location = 1) out vec2 iTexCoord;
layout(location = 2) out uint[8] iNotes;
layout(location = 10) out uint[8] iVelocities;
layout(location = 18) out uint iTime;

layout(push_constant) uniform PushConstants {
    uint note[8];
    uint velocity[8];
    uint time;
} push_constants;

void main() {

    iPosition = position;
    iTexCoord = (position+vec2(1.0))/vec2(2.0);
    iNotes = push_constants.note;
    iVelocities = push_constants.velocity;
    iTime = push_constants.time;
    gl_Position = vec4(position, 0.0, 1.0);
}