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
        let (event_tx, event_rx): (Sender<logic::game::ConrodMessage<OwnedMessage>>,
                                   Receiver<logic::game::ConrodMessage<OwnedMessage>>) =
            std::sync::mpsc::channel();
        let (render_tx, render_rx) = std::sync::mpsc::channel();
        let (proxy_tx, proxy_rx) = std::sync::mpsc::channel();
        let (proxy_action_tx, proxy_action_rx) = mpsc::channel(2); //chatview::Message
        let mut last_update = std::time::Instant::now();
        let mut gamedata = app::GameData::new();
        logic::game::GameInstance::new(Box::new(|gamedata, result_map, conrod_msg| {
            match conrod_msg.clone() {
                logic::game::ConrodMessage::Socket(j) => {
                    if let OwnedMessage::Text(z) = OwnedMessage::from(j) {
                        /*  if let Ok(s) = app::ReceivedMsg::deserialize_receive(&z) {
                                println!("s {:?}", s);
                          //      on_request::update(s, gamedata, result_map);
                            }
                            */
                    }
                }
                _ => {}
            }
        }))
                .run(&mut ui,
                     &mut (gamedata),
                     &result_map,
                     event_rx,
                     render_tx,
                     events_loop_proxy,
                     None); //proxy_action_tx
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

                last_update = std::time::Instant::now();
                last_changed += 1;
                //send to conrod
                while let Ok(s) = proxy_rx.try_recv() {
                    event_tx_clone_2.send(logic::game::ConrodMessage::Socket(s)).unwrap();
                }

            }
        });

        std::thread::spawn(move || {
                               client::run_owned_message(CONNECTION, proxy_tx, proxy_action_rx);
                           });

        let mut closed = false;
        while !closed {

            // We don't want to loop any faster than 60 FPS, so wait until it has been at least
            // 16ms since the last yield.
            let sixteen_ms = std::time::Duration::from_millis(16);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);
            println!("!!duration_since_last_update{:?},sixteenms{:?}",
                     duration_since_last_update,
                     sixteen_ms);

            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }
            events_loop.run_forever(|event| {
                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                    event_tx.send(logic::game::ConrodMessage::Event(event)).unwrap();
                }

                match event {
                    glium::glutin::Event::WindowEvent { event, .. } => match event {
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
                        },
                        // We must re-draw on `Resized`, as the event loops become blocked during
                        // resize on macOS.
                        glium::glutin::WindowEvent::Resized(..) => {
                            if let Some(primitives) = render_rx.iter().next() {
                                game_conrod::ui::draw(&display, &mut renderer, &image_map, &primitives);
                            }
                        },
                        _ => {},
                    },
                    glium::glutin::Event::Awakened => return glium::glutin::ControlFlow::Break,
                    _ => (),
                }

                glium::glutin::ControlFlow::Continue
});

            // Draw the most recently received `conrod::render::Primitives` sent from the `Ui`.
            if let Some(primitives) = render_rx.try_iter().last() {
                game_conrod::ui::draw(&display, &mut renderer, &image_map, &primitives);
            }

            last_update = std::time::Instant::now();
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
