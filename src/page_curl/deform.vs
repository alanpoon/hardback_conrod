#version 300 es
precision mediump float;
in vec3 Position;
in vec2 Texcoord0;
out vec2 v_tex_coords;
uniform float scale;
uniform float theta;
uniform float rotation;
uniform float translation;
    void main() {
        v_tex_coords = Texcoord0;
        vec3 pos = Position;
        float R = sqrt(pos.x*pos.x +pow(pos.y-translation,2.0));
        float r = R * sin (theta);
        float beta = asin (pos.x / R) / sin (theta);
        
        vec3 tmp = Position;
        tmp.x = r * sin(beta);
        tmp.y = R + translation - r * (1.0 - cos (beta)) * sin (theta);
        tmp.z = r * (1.0 - cos (beta)) * cos (theta);
        pos.x = (tmp.x * cos (rotation) - tmp.z * sin (rotation));
        pos.y = tmp.y;
        pos.z = (tmp.x * sin (rotation) + tmp.z * cos (rotation));
        pos.x = (pos.x-0.8) *scale ;
        pos.y = (pos.y-0.8) * scale;
        pos.z = pos.z *scale;
        gl_Position = vec4(pos, 1.0);
    }