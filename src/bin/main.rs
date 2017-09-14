#[macro_use]
extern crate hardback_conrod;
extern crate conrod;
extern crate conrod_chat;
extern crate futures;

use hardback_conrod as game_conrod;
use game_conrod::{app, logic};
use game_conrod::backend::{OwnedMessage, SupportIdType};
use game_conrod::backend::meta::app::{Font, ResourceEnum};
use conrod::backend::glium::glium::{self, glutin, Surface};
use hardback_conrod::page_curl::{self, page, render};
use hardback_conrod::opengl;
use conrod_chat::backend::websocket::client;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use futures::sync::mpsc;
const WIN_W: u32 = 900;
const WIN_H: u32 = 600;
const CONNECTION: &'static str = "ws://ec2-35-157-160-241.eu-central-1.compute.amazonaws.com:8080/greed";

pub struct GameApp {}

impl GameApp {
    pub fn new() -> Result<(), String> {
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new();
        let mut events_loop = glutin::EventsLoop::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        // construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).build();
        let mut result_map = HashMap::<ResourceEnum, SupportIdType>::new();
        let mut image_map = conrod::image::Map::new();
        game_conrod::ui::load_resources_to_result_map(&mut result_map,
                                                      &mut image_map,
                                                      &display,
                                                      &mut ui);
        let events_loop_proxy = events_loop.create_proxy();
        //<logic::game::ConrodMessage<OwnedMessage>>
   let (proxy_tx, proxy_rx) = std::sync::mpsc::channel();
        let (proxy_action_tx, proxy_action_rx) = mpsc::channel(2);
        let mut last_update = std::time::Instant::now();
        let mut gamedata = app::GameData::new();
        gamedata.gamestate = app::GameState::Menu;
         std::thread::spawn(move || {
                               client::run_owned_message(CONNECTION, proxy_tx, proxy_action_rx);
                           });
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
        let mut last_update = std::time::Instant::now();
        let mut game_proc =
            logic::game::GameProcess::<OwnedMessage>::new(&mut ui,Box::new(|gamedata, result_map, msg| {

            }));
        let mut events = Vec::new();
        let mut needs_update = true;
        let mut c = 0;
        'render: loop {
            let sixteen_ms = std::time::Duration::from_millis(200);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }
            events.clear();

            // Get all the new events since the last frame.
            events_loop.poll_events(|event| { events.push(event); });
            while let Ok(s) = proxy_rx.try_recv() {
                        game_proc.update_state(&mut gamedata, &result_map, s);
                }
            // Process the events.
            for event in events.drain(..) {

                // Break from the loop upon `Escape` or closed window.
                match event.clone() {

                    glium::glutin::Event::WindowEvent { event, .. } => {
                        match event {
                            glium::glutin::WindowEvent::Closed |
                            glium::glutin::WindowEvent::KeyboardInput {
                                input: glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                            } => break 'render,
                            _ => (),
                        }
                    }
                    _ => (),
                };

                // Use the `winit` backend feature to convert the winit event to a conrod input.
                let input = match conrod::backend::winit::convert_event(event, &display) {
                    None => continue,
                    Some(input) => input,
                };

                // Handle the input with the `Ui`.
                ui.handle_event(input);
                // Set the widgets.
                   game_proc.run(&mut ui, &mut (gamedata), &result_map, proxy_action_tx);

            }

            // Draw the `Ui` if it has changed.
            let primitives = ui.draw();

            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            opengl::draw_mutliple(&mut target,
                                  &vertex_buffer,
                                  &indices,
                                  &program,
                                  &mut gamedata.page_vec,
                                  &result_map);
            renderer.draw(&display, &mut target, &image_map).unwrap();

            target.finish().unwrap();
            println!("c {}", c);
            c += 1;
        }
        Ok(())
    }
}
fn main() {
    match GameApp::new() {
        Err(why) => println!("Error while running Hardback:\n{}", why),
        Ok(_) => (),
    }
}
