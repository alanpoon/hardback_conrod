extern crate hardback_conrod;
extern crate conrod;
extern crate conrod_chat;
extern crate futures;
extern crate rodio;
#[allow(non_snake_case)]
//
use hardback_conrod as game_conrod;
use game_conrod::backend::glium::{self, glutin, Surface};
use game_conrod::{app, logic};
use game_conrod::backend::{OwnedMessage, SupportIdType};
use game_conrod::backend::meta::app::{Font, ResourceEnum, AppData, MusicEnum};
use game_conrod::backend::codec_lib::codec;
use game_conrod::page_curl::{self, page, render};
use game_conrod::opengl;
use game_conrod::on_request;
use game_conrod::support;
use game_conrod::backend::codec_lib;
use game_conrod::backend::codec_lib::cards;
use game_conrod::app::BoardStruct;
use conrod_chat::backend::websocket::client;
use conrod::event;
use std::collections::HashMap;
use futures::sync::mpsc;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use rodio::{Source, Sink};

#[derive(Clone)]
pub enum ConrodMessage {
    Event(Instant, conrod::event::Input),
    Thread(Instant),
}
pub struct GameApp {}
#[cfg(feature="android")]
fn window() -> glutin::WindowBuilder {
    glutin::WindowBuilder::new()
}
#[cfg(feature="default")]
fn window() -> glutin::WindowBuilder {
    glutin::WindowBuilder::new().with_dimensions(1040, 542)
}
impl GameApp {
    pub fn new() -> Result<(), String> {
        let window_z = window();
        let context =
            glium::glutin::ContextBuilder::new()
                .with_gl(glium::glutin::GlRequest::Specific(glium::glutin::Api::OpenGlEs, (3, 0)));
        let mut events_loop = glutin::EventsLoop::new();
        let display = glium::Display::new(window_z, context, &events_loop).unwrap();
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        // construct our `Ui`.
        let (screen_w, screen_h) = display.get_framebuffer_dimensions();
        let appdata = AppData::new(screen_w as f64, screen_h as f64, "Hardback");
        let mut ui = conrod::UiBuilder::new([screen_w as f64, screen_h as f64])
            .theme(support::theme(&appdata))
            .build();

        let mut result_map = HashMap::<ResourceEnum, SupportIdType>::new();
        let mut image_map = conrod::image::Map::new();
        game_conrod::ui::init_load_resources_to_result_map(&mut result_map,
                                                           &mut image_map,
                                                           &display,
                                                           &mut ui);

        if let Some(&SupportIdType::FontId(regular)) =
            result_map.get(&ResourceEnum::Font(Font::REGULAR)) {
            ui.theme.font_id = Some(regular);
        }

        if let Some(SupportIdType::MusicId(background_music)) =
            result_map.remove(&ResourceEnum::Music(MusicEnum::BACKGROUND)) {}
        let (server_lookup_tx, server_lookup_rx) = std::sync::mpsc::channel::<Option<String>>();
        let (proxy_tx, proxy_rx) = std::sync::mpsc::channel();
        let (proxy_action_tx, proxy_action_rx) = mpsc::channel(2);
        let s_tx = Arc::new(Mutex::new(proxy_action_tx));
        let s_rx = Arc::new(Mutex::new(proxy_action_rx));
        let (ss_tx, _ss_rx) = (s_tx.clone(), s_rx.clone());
        let mut gamedata = app::GameData::new();
        let cardmeta: [codec_lib::cards::ListCard<BoardStruct>; 180] =
            cards::populate::<BoardStruct>();
        let (load_asset_tx, load_asset_rx) = std::sync::mpsc::channel();
        let mut action_instant = Instant::now(); //let the app to sleep after 1 min
        let time_to_sleep = std::time::Duration::new(15, 0);
        std::thread::spawn(move || {
            let mut last_update = std::time::Instant::now();
            let mut connected = false;
            let mut count =0;
            while !connected {
                count= count+1;
                let sixteen_ms = std::time::Duration::from_millis(1000);
                let now = std::time::Instant::now();
                let duration_since_last_update = now.duration_since(last_update);
                last_update = now;
                if duration_since_last_update < sixteen_ms {
                    std::thread::sleep(sixteen_ms - duration_since_last_update);
                }
                while let Ok(server_lookup_text_z) = server_lookup_rx.try_recv() {
                    if let Some(server_lookup_text) = server_lookup_text_z{
                        let (tx, rx) = mpsc::channel(3);
                        let mut ss_tx = ss_tx.lock().unwrap();
                        *ss_tx = tx;
                        drop(ss_tx);
                        let mut server_lookup_t = "ws://".to_owned();
                        server_lookup_t.push_str(&server_lookup_text);
                        match client::run_owned_message(server_lookup_t, proxy_tx.clone(), rx) {
                            Ok(_) => {
                                connected = true;
                                print!("Connection Success");
                            }
                            Err(_err) => {
                                connected = false;
                                print!("Connection Failure");
                            }
                        }
                        last_update = std::time::Instant::now();
                    }      
            }
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
        
        let mut game_proc =
            logic::game::GameProcess::<OwnedMessage>::new(&mut ui,
                                                          appdata,
                                                          Box::new(|gamedata,
                                                                    appdata,
                                                                    result_map,
                                                                    msg| {
                if let OwnedMessage::Text(z) = OwnedMessage::from(msg) {
                    if let Ok(s) = codec::ClientReceivedMsg::deserialize_receive(&z) {
                        on_request::update(s, gamedata, appdata, result_map);
                    } else {
                        println!("err");
                    }
                }
            }));
        let mut old_captured_event: Option<ConrodMessage> = None;
        let mut captured_event: Option<ConrodMessage> = None;
        let mut last_update = std::time::Instant::now();
        let mut gui_count=0;
        'render: loop {
            let sixteen_ms = std::time::Duration::from_millis(32);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }

            let ss_tx = s_tx.lock().unwrap();
            let proxy_action_tx = ss_tx.clone();
            let mut to_break = false;
            let mut to_continue = false;
            events_loop.poll_events(|event| {
                match event.clone() {
                    glium::glutin::Event::WindowEvent { event, .. } => {
                        match event {
                            glium::glutin::WindowEvent::CloseRequested |
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
                                  &cardmeta,
                                  &mut (gamedata),
                                  &result_map,
                                  load_asset_tx.clone(),
                                  proxy_action_tx.clone(),
                                  server_lookup_tx.clone());

                    action_instant = std::time::Instant::now();
                }
                Some(ConrodMessage::Thread(_t)) => {
                    // Set the widgets.
                    if action_instant.elapsed() <= time_to_sleep {
                        game_proc.run(&mut ui,
                                      &cardmeta,
                                      &mut (gamedata),
                                      &result_map,
                                      load_asset_tx.clone(),
                                      proxy_action_tx.clone(),
                                      server_lookup_tx.clone());
                    }

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
            while let Ok(s) = load_asset_rx.try_recv() {
                match s {
                    (ResourceEnum::Sprite(_s), Some(rgba_image), None) => {
                        let image_dimensions = rgba_image.dimensions();
                        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(),
                                                                                        image_dimensions);
                        let texture = glium::texture::Texture2d::new(&display, raw_image).unwrap();
                        let id_i = image_map.insert(texture);
                        result_map.insert(ResourceEnum::Sprite(_s), SupportIdType::ImageId(id_i));
                    }
                    (ResourceEnum::Texture(_t), Some(rgba_image), None) => {
                        let image_dimensions = rgba_image.dimensions();
                        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(),
                                                                                        image_dimensions);
                        let texture = glium::texture::Texture2d::new(&display, raw_image).unwrap();
                        result_map.insert(ResourceEnum::Texture(_t),
                                          SupportIdType::TextureId(texture));
                    }
                    (re, None, Some(_font)) => {
                        let _font_id = ui.fonts.insert(_font);
                        result_map.insert(re, SupportIdType::FontId(_font_id));
                    }
                    _ => {}
                }
            }
            // Draw the `Ui` if it has changed.
            if action_instant.elapsed() <= time_to_sleep {
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
            }

            last_update = std::time::Instant::now();
            gui_count = gui_count+1;
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
