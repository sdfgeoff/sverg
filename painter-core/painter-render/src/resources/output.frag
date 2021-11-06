#version 300 es
// Color screen based on on-screen-position

precision mediump float;
in vec2 fragCoordUV;
out vec4 FragColor;

uniform sampler2D outputTexture;


void main() {
	FragColor = texture(outputTexture, fragCoordUV);
}
