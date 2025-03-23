#version 330

uniform float z;

in vec4 position;

void main() {
    gl_Position = vec4(position.x,position.y,z,1.);
}