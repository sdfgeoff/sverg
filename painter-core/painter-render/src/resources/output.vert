#version 300 es
precision lowp float;
in vec2 aVertexPosition;
out vec2 fragCoordUV;

uniform mat3 screenToCanvas;
uniform vec2 screenResolution;
uniform vec2 canvasResolution;

void main() {
        vec2 screenAspect = vec2(screenResolution.y / screenResolution.x, 1.0);
        vec2 canvasAspect = vec2(canvasResolution.y / canvasResolution.x, 1.0);


        vec2 canvasCoords = (aVertexPosition * 2.0 - 1.0) / canvasAspect;
        
        vec2 screen_pos = (screenToCanvas * vec3(canvasCoords, 0.0)).xy;
        screen_pos = screen_pos * screenAspect;

        vec2 offset = screenToCanvas[2].xy;
        offset.x *= screenResolution.y / screenResolution.x;
        screen_pos += offset;
	fragCoordUV = aVertexPosition;
        gl_Position = vec4(
                screen_pos.xy,
                0.0,
                1.0
        );
}
