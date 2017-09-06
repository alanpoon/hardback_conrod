pub fn glsl()->&'static str{
        r#"
        #version 140
        in vec3 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        uniform float scale;
        uniform bool flipping;
        uniform float time;
            void main() {
                v_tex_coords = tex_coords;
                vec3 pos = position;
                pos.x = pos.x *scale;
                pos.y = pos.y *scale;
                pos.z = pos.z *scale;
                
                gl_Position = vec4(pos, 1.0);
            }
    "#
}