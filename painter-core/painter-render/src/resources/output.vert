#version 300 es
precision lowp float;
in vec2 aVertexPosition;
out vec2 fragCoordUV;

uniform mat3 screenToCanvas;

void main() {
        vec3 screen_pos = vec3(aVertexPosition * 2.0 - vec2(1.0), 1.0);
        screen_pos = screenToCanvas * screen_pos;
	fragCoordUV = aVertexPosition;
        gl_Position = vec4(
                screen_pos.xy,
                0.0,
                1.0
        );
}
