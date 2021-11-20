#version 300 es
precision lowp float;
in vec2 aVertexPosition;
in vec4 aStrokeData;
in vec4 aColorData;

out vec2 fragCoordUV;

uniform float aspectRatio;

out vec4 color;


void main() {
        vec2 screen_pos = aVertexPosition * 2.0 - vec2(1.0);
        screen_pos.y *= aspectRatio; // Future: needs to occur after rotation
        
        vec2 offset = aStrokeData.xy;
        float size = aStrokeData.z;

        screen_pos *= size;
        offset.x /= aspectRatio;
        screen_pos += offset;
        color = aColorData;
        

	fragCoordUV = aVertexPosition;
        gl_Position = vec4(
                screen_pos,
                0.0,
                1.0
        );
}

