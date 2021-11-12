#version 300 es
precision lowp float;
in vec2 aVertexPosition;
in vec4 aStrokeData;
out vec2 fragCoordUV;

uniform float aspectRatio;
uniform vec3 brushSize;
uniform vec3 brushFlow;

out float flow;

float evalPressureSetting(vec3 data, float pressure) {
        float min_val = data.x;
        float max_val = data.y;
        float random = data.z;  

        return min_val + pressure * (max_val - min_val) + random;
}

void main() {
        vec2 screen_pos = aVertexPosition * 2.0 - vec2(1.0);
        screen_pos.y *= aspectRatio; // Future: needs to occur after rotation
        
        float pressure = aStrokeData.z;

        float size = evalPressureSetting(brushSize, pressure);
        flow = evalPressureSetting(brushFlow, pressure);

        screen_pos *= size;
        vec2 offset = aStrokeData.xy;
        offset.x /= aspectRatio;
        screen_pos += offset;

        
        

	fragCoordUV = aVertexPosition;
        gl_Position = vec4(
                screen_pos,
                0.0,
                1.0
        );
}

