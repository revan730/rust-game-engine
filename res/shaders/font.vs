#version 330 core
layout (location = 0) in vec4 vertex;
layout (location = 1) in vec4 colour;

out vec2 TexCoords;
out vec4 FColour;

uniform mat4 translation;

void main()
{
    gl_Position = translation * vec4(vertex.xy, 0.0, 1.0);
    TexCoords = vertex.zw;
    FColour = colour;
}