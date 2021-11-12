#version 300 es
precision lowp float;
in vec2 aVertexPosition;
out vec2 fragCoordUV;

uniform mat3 screenToCanvas;
uniform vec2 screenResolution;
uniform vec2 canvasResolution;

void main() {
        vec2 canvasCoords = (aVertexPosition * 2.0 - 1.0) * canvasResolution;
        vec2 screen_pos = (screenToCanvas * vec3(canvasCoords, 0.0)).xy;
        screen_pos = screen_pos / screenResolution;
        screen_pos += screenToCanvas[2].xy;
	fragCoordUV = aVertexPosition;
        gl_Position = vec4(
                screen_pos.xy,
                0.0,
                1.0
        );
}
