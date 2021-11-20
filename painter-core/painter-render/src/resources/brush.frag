#version 300 es
// Color screen based on on-screen-position

precision mediump float;
in vec2 fragCoordUV;
out vec4 FragColor;

uniform sampler2D brushTexture;

in vec4 color;


void main() {
	vec4 outCol = texture(brushTexture, fragCoordUV);
	outCol *= color;

	FragColor = outCol;
}
