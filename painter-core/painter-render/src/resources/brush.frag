#version 300 es
// Color screen based on on-screen-position

precision mediump float;
in vec2 fragCoordUV;
out vec4 FragColor;


void main() {
	FragColor = vec4(
		fragCoordUV.x,
		fragCoordUV.y,
		1.0,
		1.0
	);
}
