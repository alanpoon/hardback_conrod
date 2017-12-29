extern crate hardback_conrod;
extern crate conrod;
extern crate conrod_chat;
extern crate futures;
extern crate toa_ping;

#[allow(non_snake_case)]
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
use game_conrod::backend::codec_lib;
use game_conrod::backend::codec_lib::cards;
use game_conrod::ui::{Vala,load_resources_iter,iter_resource_enum_vala_next};
use game_conrod::app::{LoadAssetStatus, BoardStruct};
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
#[cfg(target_os="android")]
fn window() -> glutin::WindowBuilder {
    glutin::WindowBuilder::new()
}
#[cfg(target_os="linux")]
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

        let (proxy_tx, proxy_rx) = std::sync::mpsc::channel();
        let (proxy_action_tx, proxy_action_rx) = mpsc::channel(2);
        let s_tx = Arc::new(Mutex::new(proxy_action_tx));
        let s_rx = Arc::new(Mutex::new(proxy_action_rx));
        let (ss_tx, _ss_rx) = (s_tx.clone(), s_rx.clone());
        let mut gamedata = app::GameData::new();
        gamedata.guistate = app::GuiState::Menu;
        let cardmeta: [codec_lib::cards::ListCard<BoardStruct>; 180] =
            cards::populate::<BoardStruct>();
        let need_to_load_asset = Arc::new(Mutex::new(LoadAssetStatus::NOSTART));
        let need_to_load_asset_c = need_to_load_asset.clone();
        let (load_asset_tx,load_asset_rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {

            let mut events_loop = glutin::EventsLoop::new();

            let mut connected = false;
            let mut last_update = std::time::Instant::now();
            let mut c = 0;
            let map:HashMap<ResourceEnum,Vala> =HashMap::new();
           
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
            let mut events_loop = glutin::EventsLoop::new();
            while connected {
                let sixteen_ms = std::time::Duration::from_millis(1000);
                let now = std::time::Instant::now();
                let duration_since_last_update = now.duration_since(last_update);
                if (duration_since_last_update < sixteen_ms) & (c > 0) {
                    std::thread::sleep(sixteen_ms - duration_since_last_update);
                }
                let mut need_to_load_asset_ = need_to_load_asset.lock().unwrap();
                if let LoadAssetStatus::START = *need_to_load_asset_ {
                    let windowz=window();
                    let context = glium::glutin::ContextBuilder::new()
                        .with_gl(glium::glutin::GlRequest::Specific(glium::glutin::Api::OpenGlEs,
                                                                    (3, 0)));
                    let display = glium::Display::new(window_z, context, &events_loop).unwrap();
                     load_resources_iter(&mut map);
                     let mut _iter_ResourceEnum_Vala =map.iter();
                    while let Some((ref k,ref v)) = _iter_ResourceEnum_Vala.next(){
                        let _send_this = iter_resource_enum_vala_next(&display,*k.clone(),*v.clone());
                        load_asset_tx.send(_send_this).unwrap();
                    }
                                                                   
                    *need_to_load_asset_ = LoadAssetStatus::DONE;
                } else {
                    drop(need_to_load_asset_);
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
                                  &cardmeta,
                                  &mut (gamedata),
                                  &result_map,
                                  need_to_load_asset_c.clone(),
                                  proxy_action_tx.clone());

                }
                Some(ConrodMessage::Thread(_t)) => {
                    // Set the widgets.
                    game_proc.run(&mut ui,
                                  &cardmeta,
                                  &mut (gamedata),
                                  &result_map,
                                  need_to_load_asset_c.clone(),
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
            while let Ok(s) =load_asset_rx.try_recv(){
                match s {
                    (ResourceEnum::Sprite(_s),Some(_texture),None)=>{
                         let id_i = image_map.insert(_texture);
                        result_map.insert(ResourceEnum::Sprite(_s),SupportIdType::ImageId(id_i));
                    },
                    (ResourceEnum::Texture(_t),Some(_texture),None)=>{
                        result_map.insert(ResourceEnum::Texture(_t),SupportIdType::TextureId(_texture));
                    },
                    (re,None,Some(_font))=>{
                        let _font_id = ui.fonts.insert(_font);
                         result_map.insert(re,SupportIdType::FontId(_font_id));
                    }
                }
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
