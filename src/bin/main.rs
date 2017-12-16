extern crate hardback_conrod;
extern crate conrod;
extern crate conrod_chat;
extern crate futures;
extern crate toa_ping;

use hardback_conrod as game_conrod;
use game_conrod::backend::glium::{self, glutin, Surface};
use game_conrod::{app, logic};
use game_conrod::backend::{OwnedMessage, SupportIdType};
use game_conrod::backend::meta::app::{Font, ResourceEnum, AppData};
use game_conrod::backend::codec_lib::codec;
use game_conrod::page_curl::{self, page, render};
use game_conrod::opengl;
use game_conrod::on_request;
use game_conrod::support;
use conrod_chat::backend::websocket::client;
use conrod::event;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use futures::sync::mpsc;
use std::time::Instant;

#[cfg(target_os="android")]
const CONNECTION: &'static str = "ws://13.229.94.195:8080";
#[cfg(not(target_os="android"))]
const CONNECTION: &'static str = "ws://0.0.0.0:8080";
#[derive(Clone)]
pub enum ConrodMessage {
    Event(Instant, conrod::event::Input),
    Thread(Instant),
}
pub struct GameApp {}

impl GameApp {
    pub fn new() -> Result<(), String> {
        let window = glutin::WindowBuilder::new();
        let context =
            glium::glutin::ContextBuilder::new()
                .with_gl(glium::glutin::GlRequest::Specific(glium::glutin::Api::OpenGlEs, (3, 0)));
        let mut events_loop = glutin::EventsLoop::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        // construct our `Ui`.
        let (screen_w, screen_h) = display.get_framebuffer_dimensions();
        let appdata = AppData::new(screen_w as f64, screen_h as f64, "Hardback");
        let mut ui = conrod::UiBuilder::new([screen_w as f64, screen_h as f64])
            .theme(support::theme(&appdata))
            .build();
        let mut result_map = HashMap::<ResourceEnum, SupportIdType>::new();
        let mut image_map = conrod::image::Map::new();
        game_conrod::ui::load_resources_to_result_map(&mut result_map,
                                                      &mut image_map,
                                                      &display,
                                                      &mut ui);
        if let Some(&SupportIdType::FontId(regular)) =
            result_map.get(&ResourceEnum::Font(Font::REGULAR)) {
            ui.theme.font_id = Some(regular);
        }

        //<logic::game::ConrodMessage<OwnedMessage>>
        let (proxy_tx, proxy_rx) = std::sync::mpsc::channel();
        let (proxy_action_tx, proxy_action_rx) = mpsc::channel(2);
        let s_tx = Arc::new(Mutex::new(proxy_action_tx));
        let s_rx = Arc::new(Mutex::new(proxy_action_rx));
        let (ss_tx, _ss_rx) = (s_tx.clone(), s_rx.clone());
        let mut gamedata = app::GameData::new();
        gamedata.guistate = app::GuiState::Menu;
        std::thread::spawn(move || {
            let mut connected = false;
            let mut last_update = std::time::Instant::now();
            let mut c = 0;
            while !connected {
                let sixteen_ms = std::time::Duration::from_millis(500);
                let now = std::time::Instant::now();
                let duration_since_last_update = now.duration_since(last_update);
                if (duration_since_last_update < sixteen_ms) & (c > 0) {
                    std::thread::sleep(sixteen_ms - duration_since_last_update);
                }
                match toa_ping::run("www.google.com") {
                    Ok(_) => {
                        println!("internet connection");
                        let (tx, rx) = mpsc::channel(3);
                        let mut ss_tx = ss_tx.lock().unwrap();
                        *ss_tx = tx;
                        drop(ss_tx);
                        match client::run_owned_message(CONNECTION, proxy_tx.clone(), rx) {
                            Ok(_) => {
                                println!("connected");
                                connected = true;
                            }
                            Err(_err) => {
                                println!("reconnecting");
                                connected = false;
                            }
                        }

                    }
                    _ => {
                        /*for test*/
                        let (tx, rx) = mpsc::channel(3);
                        let mut ss_tx = ss_tx.lock().unwrap();
                        *ss_tx = tx;
                        drop(ss_tx);
                        match client::run_owned_message(CONNECTION, proxy_tx.clone(), rx) {
                            Ok(_) => {
                                println!("connected");
                                connected = true;
                            }
                            Err(_err) => {
                                println!("reconnecting");
                                connected = false;
                            }
                        }
                        //connected = false;
                    }
                }
                last_update = std::time::Instant::now();
                c += 1;
            }

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
            logic::game::GameProcess::<OwnedMessage>::new(&mut ui,
                                                          appdata,
                                                          Box::new(|gamedata,
                                                                    appdata,
                                                                    result_map,
                                                                    msg| {
                if let OwnedMessage::Text(z) = OwnedMessage::from(msg) {
                    if let Ok(s) = codec::ClientReceivedMsg::deserialize_receive(&z) {
                        println!("s {:?}", s);
                        on_request::update(s, gamedata, appdata, result_map);
                    } else {
                        println!("err");
                    }
                }
            }));
        let mut old_captured_event: Option<ConrodMessage> = None;
        let mut captured_event: Option<ConrodMessage> = None;
        let sixteen_ms = std::time::Duration::from_millis(800);

        'render: loop {
            let ss_tx = s_tx.lock().unwrap();
            let proxy_action_tx = ss_tx.clone();
            let mut to_break = false;
            let mut to_continue = false;
            events_loop.poll_events(|event| {
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
                            } => {to_break=true;}
                            _ => (),
                        }
                    }
                    _ => {}
                }
                match conrod::backend::winit::convert_event(event.clone(), &display) {
                    None => {
                        to_continue = true;
                    }
                    Some(input) => {
                        let d = std::time::Instant::now();
                        if let event::Input::Text(s) = input.clone() {
                            if s != String::from("") {
                                captured_event = Some(ConrodMessage::Event(d, input));
                            }
                        } else {
                            captured_event = Some(ConrodMessage::Event(d, input));
                        }

                    }
                }
            });
            if to_break {
                break 'render;
            }
            if to_continue {
                continue;
            }
            match captured_event {
                Some(ConrodMessage::Event(d, ref input)) => {
                    if let Some(ConrodMessage::Event(_oldd, ref oldinput)) = old_captured_event {
                        if oldinput.clone() != input.clone() {
                            ui.handle_event(input.clone());
                        }
                    }
                    if let None = old_captured_event {
                        ui.handle_event(input.clone());
                    }
                    old_captured_event = Some(ConrodMessage::Event(d, input.clone()));
                    // Set the widgets.

                    game_proc.run(&mut ui,
                                  &mut (gamedata),
                                  &result_map,
                                  proxy_action_tx.clone());

                }
                Some(ConrodMessage::Thread(_t)) => {
                    // Set the widgets.
                    game_proc.run(&mut ui,
                                  &mut (gamedata),
                                  &result_map,
                                  proxy_action_tx.clone());
                }
                None => {
                    let now = std::time::Instant::now();
                    let duration_since_last_update = now.duration_since(last_update);
                    if duration_since_last_update < sixteen_ms {
                        std::thread::sleep(sixteen_ms - duration_since_last_update);
                    }
                    let t = std::time::Instant::now();
                    captured_event = Some(ConrodMessage::Thread(t));
                }
            }

            while let Ok(s) = proxy_rx.try_recv() {
                game_proc.update_state(&mut gamedata, &result_map, s);
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
