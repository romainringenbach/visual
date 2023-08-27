layout(location = 0) out vec4 f_color;
layout(location = 0) in vec2 iPosition;
layout(location = 1) in vec2 iTexCoord;
layout(location = 2) flat in uint[8] iNotes;
layout(location = 10) flat in uint[8] iVelocities;
layout(location = 18) flat in uint iTime;

layout(set = 0, binding = 0) uniform sampler2D tex;

