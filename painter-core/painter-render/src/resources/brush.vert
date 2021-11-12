#version 300 es
precision lowp float;
in vec2 aVertexPosition;
in vec4 aStrokeData;
out vec2 fragCoordUV;

void main() {
        vec2 screen_pos = aVertexPosition * 2.0 - vec2(1.0);
        screen_pos *= 0.01;
        screen_pos += aStrokeData.xy;
	fragCoordUV = aVertexPosition;
        gl_Position = vec4(
                screen_pos,
                0.0,
                1.0
        );
}
