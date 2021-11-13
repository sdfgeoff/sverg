#version 300 es
// Color screen based on on-screen-position

precision mediump float;
in vec2 fragCoordUV;
out vec4 FragColor;

uniform sampler2D brushTexture;

in float flow;

uniform vec4 brushColor;

void main() {
	
	// brush_shape should come from a texture, but for now this will do
	vec4 outCol = texture(brushTexture, fragCoordUV);
	outCol.a *= flow;
	outCol *= brushColor;

	FragColor = outCol;
}
