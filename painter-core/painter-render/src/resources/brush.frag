#version 300 es
// Color screen based on on-screen-position

precision mediump float;
in vec2 fragCoordUV;
out vec4 FragColor;

in float flow;

uniform vec4 brushColor;

void main() {
	
	// brush_shape should come from a texture, but for now this will do
	float brush_shape = clamp(1.0 - length(fragCoordUV * 2.0 - 1.0), 0.0, 1.0);

	vec4 outCol = brushColor;
	outCol.a *= brush_shape * flow;

	FragColor = outCol;
}
