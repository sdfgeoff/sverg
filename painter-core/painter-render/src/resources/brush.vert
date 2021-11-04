#version 300 es
precision lowp float;
in vec2 aVertexPosition;
out vec2 fragCoordUV;

void main() {
        vec2 screen_pos = aVertexPosition * 2.0 - vec2(1.0);
	fragCoordUV = aVertexPosition;
        gl_Position = vec4(
                screen_pos,
                0.0,
                1.0
        );
}
