extern crate conrod;
#[macro_use]
extern crate glium;
extern crate hardback_conrod;
fn main() {
    use hardback_conrod::page_curl::{page,render};

    use conrod::backend::glium::glium::{self, glutin, Surface};
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut _page =page::Page::new();
    {
         render(&mut _page);
    }
   
   // println!("shape {:?}",shape);
   let cx = 20 + 1;
    let cy = 25 + 1;

      let stripLen =1050 + 24;
   fn vector_as_u8_4_array(vector: &Vec<u16>) -> [u16;1074] {
    let mut arr = [0u16;1074];
    for i in 0..1074 {
        arr[i] = vector[i];
    }
    arr
}
//let arr_indices = vector_as_u8_4_array(&_page.front_strip);
    let vertex_buffer = glium::VertexBuffer::new(&display, &_page.out_mesh).unwrap();
let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TriangleStrip,
                                      &_page.front_strip).unwrap();
    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        uniform float scale;
            void main() {
                vec3 pos = position;
                pos.x = pos.x *scale;
                pos.y = pos.y *scale;
                pos.z = pos.z *scale;
                gl_Position = vec4(pos, 1.0);
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

        let uniforms = uniform! { scale: 1.0f32 };
        target.draw(&vertex_buffer,
                    &indices,
                    &program,
                     &uniforms,
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
