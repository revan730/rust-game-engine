#version 330
out vec4 FragColor;

in vec2 TexCoords;
in vec4 FColour;

uniform sampler2D tex;

void main() {
    FragColor = FColour * vec4(1.0, 1.0, 1.0, texture(tex, TexCoords).r);
}
