pub fn glsl() -> &'static str {
    r#"
        #version 300 es
        precision mediump float;
        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;
        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#
}
