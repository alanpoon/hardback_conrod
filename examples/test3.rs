#[macro_use]
extern crate glium;
#[macro_use]
extern crate conrod;
extern crate hardback_conrod;
extern crate image;
extern crate hardback_meta;
extern crate find_folder;
extern crate websocket;
use self::hardback_meta::app::{ResourceEnum, Font, Sprite};
use glium::{glutin, Surface};
use hardback_conrod::page_curl::{self, page, render};
use hardback_conrod::opengl;
use hardback_conrod::app;
use hardback_conrod::logic;
use hardback_conrod::logic::game::SupportIdType;
use std::collections::HashMap;
use conrod_chat::backend::websocket::client;
#[macro_use]
mod support;
use std::io::Cursor;
const WIN_W: u32 = 900;
const WIN_H: u32 = 600;
fn main() {
    use conrod;
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let image = image::load(Cursor::new(&include_bytes!("../assets/images/back512.png")[..]),
                            image::PNG)
            .unwrap()
            .to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(),
                                                                   image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    let mut _page = page::Page::new();
    {
        render(&mut _page);
    }
    fn draw(display: &glium::Display,
            renderer: &mut conrod::backend::glium::Renderer,
            image_map: &conrod::image::Map<glium::Texture2d>,
            primitives: &conrod::render::OwnedPrimitives) {
        renderer.fill(display, primitives.walk(), &image_map);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer.draw(display, &mut target, &image_map).unwrap();
        target.finish().unwrap();
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

    let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).build();
    image_map!{
    (ResourceEnum::Font(Font::REGULAR),"font","fonts/NotoSans/NotoSans-Regular.ttf"),
     (ResourceEnum::Font(Font::BEON),"font","fonts/Beon/beon-webfont.ttf")
  }
    let mut image_map = conrod::image::Map::new();
    let g = ImageIds::new();
    let mut result_map = HashMap::<ResourceEnum, SupportIdType>::new();
    g.pump(&mut result_map, &display, &mut ui, &mut image_map);
    let (event_tx, event_rx) = std::sync::mpsc::channel();
    let (render_tx, render_rx) = std::sync::mpsc::channel();
    let mut t: f32 = -0.5;
    let mut closed = false;
    let mut gamedata = app::GameData::new();
    std::thread::spawn(move || {
        logic::game::GameInstance::new(Box::new(|gamedata, result_map, conrod_msg| {
            match conrod_msg.clone() {
                logic::game::ConrodMessage::Socket(j) => {
                    if let websocket::OwnedMessage::Text(z) = websocket::OwnedMessage::from(j) {
                        if let Ok(s) = app::ReceivedMsg::deserialize_receive(&z) {
                            println!("s {:?}", s);
                            // on_request::update(s, gamedata, result_map);
                        }
                    }
                }
                _ => {}
            }

        }))
                .run(&mut ui,
                     &mut gamedata,
                     &result_map,
                     event_rx,
                     proxy_action_tx,
                     render_tx,
                     events_loop_proxy);
    });
    let (proxy_tx, proxy_rx) = std::sync::mpsc::channel();
    let event_tx_clone_2 = event_tx.clone();
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
            while let Ok(s) = proxy_rx.try_recv() {
                event_tx_clone_2.send(logic::game::ConrodMessage::Socket(s)).unwrap();
            }

        }
    });

    std::thread::spawn(move || { client::run(CONNECTION, proxy_tx, proxy_action_rx); });

    while !closed {
        // we update `t`
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        _page.update_time();
        let uniforms = uniform! { scale: 1.0f32,tex:&texture,rotation:_page.rotation,
        translation:_page.translation,
        theta:_page.theta };
        opengl::draw(&display, &vertex_buffer, &indices, &program, &uniforms);
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
