#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform float hallo;

void main() {
    gl_FragColor = texture2D(Texture, uv);
}