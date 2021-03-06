#version 150 core

uniform sampler2D t_Texture;

in vec2 v_TexCoord;

out vec4 Target0;

void main() {
    vec4 colour = texture(t_Texture, v_TexCoord);
    if (colour.a < 0.001) {
        discard;
    }
    Target0 = colour;
}
