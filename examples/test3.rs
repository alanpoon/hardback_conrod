extern crate conrod;
#[macro_use]
extern crate glium;
extern crate hardback_conrod;
extern crate image;

use std::io::Cursor;
fn main() {
    use hardback_conrod::page_curl::{self,page,render};

    use conrod::backend::glium::glium::{self, glutin, Surface};
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let image = image::load(Cursor::new(&include_bytes!("../assets/images/back512.png")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    let mut _page =page::Page::new();
    {
         render(&mut _page);
    }
   

//let arr_indices = vector_as_u8_4_array(&_page.front_strip);
    let vertex_buffer = glium::VertexBuffer::new(&display, &_page.out_mesh).unwrap();
let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TriangleStrip,
                                      &_page.front_strip).unwrap();
    let vertex_shader_src=page_curl::deform::glsl();

    let fragment_shader_src = page_curl::fragment::glsl();

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

        let uniforms = uniform! { scale: 1.0f32,tex:&texture };
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
