#version 330 core

in VERT_SHAD_OUTPUT {
    vec3 Color;
    vec2 TexCoord;
} IN;

uniform float timed_colour;
uniform sampler2D ourTexture;

out vec4 Color;

void main()
{
    // Color = vec4(IN.Color, 1.0f);    // Positional colour
    // Color = vec4(0.0f, abs(sin(timed_colour)), 0.0f, 1.0f);   // Varying green colour
    Color = texture(ourTexture, IN.TexCoord);  // From texture
}