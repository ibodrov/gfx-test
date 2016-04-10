#version 150 core

in ivec2 a_Pos;
in vec4 a_Color;

uniform mat4 u_Transform;

out vec4 v_Color;

void main() {
    gl_Position = u_Transform * vec4(a_Pos, 0.0, 1.0);
    v_Color = a_Color;
}
