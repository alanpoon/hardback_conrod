#[macro_use]
extern crate glium;
#[macro_use]
extern crate conrod;
#[macro_use]
extern crate cardgame_macros;
extern crate hardback_conrod;
extern crate image;
extern crate hardback_meta;
extern crate find_folder;
extern crate websocket;
use glium::{glutin, Surface};
use hardback_conrod::page_curl::{self, page, render};
use hardback_conrod::opengl;
use hardback_conrod::app;
use hardback_conrod as game_conrod;
use hardback_meta as game_meta;

#[macro_use]
mod support;

const WIN_W: u32 = 900;
const WIN_H: u32 = 600;
fn main() {
    use conrod;
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let mut events_loop = glutin::EventsLoop::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let mut _page = page::Page::new();
    {
        render(&mut _page);
    }


    let vertex_buffer = glium::VertexBuffer::new(&display, &_page.in_mesh).unwrap();
    let indices = glium::IndexBuffer::new(&display,
                                          glium::index::PrimitiveType::TriangleStrip,
                                          &_page.front_strip)
            .unwrap();
    let vertex_shader_src = page_curl::deform::glsl();
    let fragment_shader_src = page_curl::fragment::glsl();
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    image_map!{
     (ResourceEnum::Sprite(Sprite::BOXCLOSURE),"image","images/back512.png"), 
    (ResourceEnum::Font(Font::REGULAR),"font","fonts/NotoSans/NotoSans-Regular.ttf"),
     (ResourceEnum::Font(Font::BEON),"font","fonts/Beon/beon-webfont.ttf")
  }

    CGM_game!{};

    std::thread::spawn(move || {
        let mut last_update = std::time::Instant::now();
        let sixteen_ms = std::time::Duration::from_millis(1000);
        let mut last_changed = 0;
        'test: loop {
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);
            println!("duration_since_last_update{:?},sixteenms{:?}",
                     duration_since_last_update,
                     sixteen_ms);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }
            println!("last_changed {}", last_changed);
            /*   event_tx_clone_2.send(logic::game::ConrodMessage::Animate(app::AnimateMsg::Loading(last_changed)))
                                       .unwrap();
                                       */
            last_update = std::time::Instant::now();
            last_changed += 1;
            //send to conrod
           /*     while let Ok(s) = proxy_rx.try_recv() {
                    event_tx_clone_2.send(logic::game::ConrodMessage::Socket(s)).unwrap();
                }
                */
        }
    });

    let mut closed = false;
    while !closed {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        opengl::draw(&display, &vertex_buffer, &indices, &program, &_page);
        events_loop.run_forever(|event| {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                event_tx.send(logic::game::ConrodMessage::Event(event)).unwrap();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::WindowEvent::Closed |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => {
                            closed = true;
                            return glium::glutin::ControlFlow::Break;
                        }
                        // We must re-draw on `Resized`, as the event loops become blocked during
                        // resize on macOS.
                        glium::glutin::WindowEvent::Resized(..) => {
                            if let Some(primitives) = render_rx.iter().next() {
                                draw(&display, &mut renderer, &image_map, &primitives);
                            }
                        }
                        _ => {}
                    }
                }
                glium::glutin::Event::Awakened => return glium::glutin::ControlFlow::Break,
                _ => (),
            }

            glium::glutin::ControlFlow::Continue
        });

    }
}
