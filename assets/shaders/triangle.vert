#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;

out VERT_SHAD_OUTPUT {
    vec2 TexCoord;
} OUT;

void main()
{
    gl_Position = vec4(Position, 1.0);
    OUT.TexCoord = TexCoord;
}