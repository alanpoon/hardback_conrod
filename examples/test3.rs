extern crate conrod;
extern crate hardback_conrod;
fn main() {
    use hardback_conrod::page_curl::{page,render};

    use conrod::backend::glium::glium::{self, glutin, Surface};
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut _page =page::Page::new();
    let shape = render(&mut _page);

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = -0.5;
    let mut closed = false;
    while !closed {
        // we update `t`
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
      //  let uniforms = uniform! { t: t };
        target.draw(&vertex_buffer,
                    &indices,
                    &program,
                     &glium::uniforms::EmptyUniforms,
                    &Default::default())
            .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
                                    glutin::Event::WindowEvent { event, .. } => {
                                        match event {
                                            glutin::WindowEvent::Closed => closed = true,
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                });
    }
}
//cargo run --release --features=\"winit glium
