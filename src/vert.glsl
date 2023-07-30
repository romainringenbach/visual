#version 450

layout(location = 0) in vec2 position;
layout(location = 0) out vec2 iPosition;
layout(location = 1) out vec2 iTexCoord;

void main() {
    iPosition = position;
    iTexCoord = (position+vec2(1.0))/vec2(2.0);
    gl_Position = vec4(position, 0.0, 1.0);
}