#[cfg(all(feature="backend_glium_winit"))]
#[macro_use]
extern crate conrod;
#[macro_use]
mod support;
#[macro_use]
extern crate greed_conrod;

extern crate websocket;
fn main() {
    feature::main();
}
#[allow(non_snake_case)]
#[cfg(feature="backend_glium_winit")]
mod feature {
    extern crate greed_meta;
    extern crate conrod_chat;
    use support::image_macros;
    use greed_conrod::logic::game::SupportIdType;
    use greed_conrod::{app, logic};
    use greed_conrod::on_request;
    use feature::conrod_chat::chat;
    use feature::conrod_chat::backend::websocket::client;
    extern crate find_folder;
    extern crate image;
    extern crate futures;
    use websocket;
    use conrod;
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::Surface;
    use self::greed_meta::app::{ResourceEnum, Font, Sprite};
    const WIN_W: u32 = 900;
    const WIN_H: u32 = 600;
    const CONNECTION: &'static str = "ws://ec2-35-157-160-241.eu-central-1.compute.amazonaws.com:8080/greed";
    use self::futures::{Future, Sink};
    use self::futures::sync::mpsc;
    use std;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    pub fn main() {
        // Build the window.
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("Conrod with glium!")
            .with_dimensions(WIN_W, WIN_H);
        let context = glium::glutin::ContextBuilder::new().with_vsync(true).with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
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
        let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).build();
        image_map!{
       (ResourceEnum::Sprite(Sprite::BROWNPAPER),"image",greed_meta::app::get_brownpaper_path()),  
    (ResourceEnum::Sprite(Sprite::TRANSPARENT),"image",greed_meta::app::get_transparent_path()), 
       (ResourceEnum::Sprite(Sprite::BOXCLOSURE),"image",greed_meta::app::get_boxclosure_path()), 
        (ResourceEnum::Sprite(Sprite::FRAME),"image",greed_meta::app::get_frame_path()),     
     (ResourceEnum::Sprite(Sprite::BUTTON),"image",greed_meta::app::get_button_path()),     
    (ResourceEnum::Sprite(Sprite::ICONS),"image",greed_meta::app::get_icons_path()),
    (ResourceEnum::Sprite(Sprite::CARDS),"image",greed_meta::app::get_cards_path()),
  //    (ResourceEnum::Sprite(Sprite::PAGE2),"image",greed_meta::app::get_page2_path()),
    (ResourceEnum::Sprite(Sprite::RUST),"image",greed_meta::app::get_rust_path()),
    (ResourceEnum::Font(Font::REGULAR),"font","fonts/NotoSans/NotoSans-Regular.ttf"),
     (ResourceEnum::Font(Font::BEON),"font","fonts/Beon/beon-webfont.ttf")
  }
        let mut image_map = conrod::image::Map::new();
        let g = ImageIds::new();
        let mut result_map = HashMap::<ResourceEnum, SupportIdType>::new();
        g.pump(&mut result_map, &display, &mut ui, &mut image_map);
        let (event_tx, event_rx) = std::sync::mpsc::channel();
        let (render_tx, render_rx) = std::sync::mpsc::channel();
        // This window proxy will allow conrod to wake up the `winit::Window` for rendering.
        let events_loop_proxy = events_loop.create_proxy();
        let mut last_update = std::time::Instant::now();
        let mut gamedata = app::GameData::new();
        //gamedata.gamestate = app::GameState::Start;
        //    gamedata.gamestate = app::GameState::Lobby(true);
        gamedata.gamestate = app::GameState::Tutorial;
        gamedata.tutorialstate = app::TutorialState::GANGLAND;
        gamedata.top_left = app::TopLeft::hand;
        gamedata.private_info = Some(app::PrivateInformation {
                                         hand: vec![app::Card {
                                                        timingNumber: 40,
                                                        markers: None,
                                                    },
                                                    app::Card {
                                                        timingNumber: 50,
                                                        markers: None,
                                                    }],
                                         draft_pile: vec![],
                                         position: 0,
                                     });
        gamedata.log = "asdsa".to_owned();
        gamedata.footer = app::Footer::ShowPrivate;
        gamedata.players = vec![app::Player {
                                    name: "adasdsds".to_owned(),
                                    cash: 2000,
                                    cars: 1,
                                    guns: 1,
                                    keys: 1,
                                    hearts: 1,
                                    bottles: 1,
                                    wrenches: 1,
                                    holdings: vec![app::Card {
                                                       timingNumber: 50,
                                                       markers: None,
                                                   }],
                                    thugs: vec![],
                                    actions: vec![],
                                }];
        let jGAME = Arc::new(Mutex::new(gamedata));
        let mut jConnection = Arc::new(Mutex::new(false));
        let (proxy_action_tx, proxy_action_rx) = mpsc::channel(2); //chatview::Message
        //   let mut handles = vec![];
        let jGAME1 = jGAME.clone();
        std::thread::spawn(move || {
            let mut _gamedata = jGAME1.lock().unwrap();
            let cj = logic::game::GameInstance::new(Box::new(|gamedata,
                                                              result_map,
                                                              conrod_msg| {
                match conrod_msg.clone() {
                    logic::game::ConrodMessage::Socket(j) => {
                        if let websocket::OwnedMessage::Text(z) = websocket::OwnedMessage::from(j) {
                            if let Ok(s) = app::ReceivedMsg::deserialize_receive(&z) {
                                println!("s {:?}", s);
                                on_request::update(s, gamedata, result_map);
                            }
                        }
                    }
                    logic::game::ConrodMessage::Animate(j) => {
                        on_request::animate(j, gamedata, result_map);
                    }
                    _ => {}
                }

            }))
                    .run(&mut ui,
                         &mut (*_gamedata),
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
                event_tx_clone_2.send(logic::game::ConrodMessage::Animate(app::AnimateMsg::Loading(last_changed)))
                                       .unwrap();
                last_update = std::time::Instant::now();
                last_changed += 1;
                //send to conrod
                while let Ok(s) = proxy_rx.try_recv() {
                    event_tx_clone_2.send(logic::game::ConrodMessage::Socket(s)).unwrap();
                }

            }
        });

        std::thread::spawn(move || { client::run(CONNECTION, proxy_tx, proxy_action_rx); });

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
                                draw(&display, &mut renderer, &image_map, &primitives);
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
                draw(&display, &mut renderer, &image_map, &primitives);
            }

            last_update = std::time::Instant::now();
        }
    }

}
