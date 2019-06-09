extern crate hardback_conrod;
extern crate conrod_core;
extern crate conrod_crayon;
extern crate conrod_chat;
extern crate futures;
extern crate rodio;
extern crate crayon;
extern crate crayon_bytes;
#[allow(non_snake_case)]
//
use conrod_core::text::{FontCollection};

use hardback_conrod as game_conrod;
use game_conrod::{app, logic};
use game_conrod::backend::{SupportIdType};
use game_conrod::backend::meta::app::{Font, ResourceEnum, AppData, MusicEnum};
use game_conrod::backend::codec_lib::codec;
use game_conrod::page_curl::{self, page, render};
//use game_conrod::opengl;
use game_conrod::on_request;
use game_conrod::support;
use game_conrod::backend::codec_lib;
use game_conrod::backend::codec_lib::cards;
use game_conrod::backend::WindowResources;
use game_conrod::app::BoardStruct;
use conrod_chat::backend::websocket::client;
use conrod_crayon::Renderer;
use crayon::prelude::*;
use crayon_bytes::prelude::*;
use crayon::network;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone)]
pub enum ConrodMessage {
    Event(Instant, conrod_core::event::Input),
    Thread(Instant),
}
const WIN_W: u32 = 600;
const WIN_H: u32 = 420;

struct Window {
    action_instant:Instant,
    cardmeta: [codec_lib::cards::ListCard<BoardStruct>; 180],
    game_process:logic::game::GameProcess<String>,
    game_data: app::GameData,
    renderer: Renderer,
    result_map:HashMap::<ResourceEnum, SupportIdType>,
    ui: conrod_core::Ui,
    image_map: conrod_core::image::Map<TextureHandle>,
    batch: CommandBuffer,
    time: f32,
    page:page::Page,
    resources: WindowResources,
}
//crayon_bytes = { git = "https://github.com/alanpoon/crayon.git", branch ="textedit"}
//crayon = { git = "https://github.com/alanpoon/crayon.git", branch ="textedit"}

impl Window {
    pub fn build(resources: &WindowResources) -> CrResult<Self> {
        let screen_dim = crayon::window::dimensions();
        let (screen_w,screen_h) = (screen_dim.x,screen_dim.y);
        let appdata = AppData::new(screen_w as f64, screen_h as f64, "Hardback");
        let mut ui = conrod_core::UiBuilder::new([screen_w as f64, screen_h as f64])
         //   .theme(support::theme(&appdata))
            .build();
        let mut result_map = HashMap::<ResourceEnum, SupportIdType>::new();
        let mut image_map = conrod_core::image::Map::new();
        resources.pump(&mut result_map,&mut image_map,&mut ui);
        println!("result_map {:?}",result_map.len());
        if let Some(&SupportIdType::FontId(regular)) =
            result_map.get(&ResourceEnum::Font(Font::REGULAR)) {
            ui.theme.font_id = Some(regular);
        }
        let dpi_factor = crayon::window::device_pixel_ratio();
        println!("dpi_factor {:?}",dpi_factor);
        let renderer = Renderer::new((screen_w as f64,screen_h as f64),  dpi_factor as f64);
        let gamedata = app::GameData::new();
        let cardmeta: [codec_lib::cards::ListCard<BoardStruct>; 180] =
            cards::populate::<BoardStruct>();
        let game_proc =
            logic::game::GameProcess::<String>::new(app::Ids::new(ui.widget_id_generator()),
                                                          appdata,
                                                          Box::new(|gamedata,
                                                                    appdata,
                                                                    result_map,
                                                                    msg| {
                    if let Ok(s) = codec::ClientReceivedMsg::deserialize_receive(&msg) {
                        on_request::update(s, gamedata, appdata, result_map);
                    } else {
                        println!("err");
                    }
            }));
        let mut page = page::Page::new();
        {
            render(&mut page);
        }
        let action_instant = Instant::now();
        Ok(Window {
            action_instant:action_instant,
            cardmeta:cardmeta,
            game_data:gamedata,
            game_process: game_proc,
            ui:ui,
            image_map:image_map,
            renderer:renderer,
            result_map:result_map,
            batch: CommandBuffer::new(),
            time: 0.0,
            page:page,
            resources: *resources
        })
    }
}

impl Drop for Window {
    fn drop(&mut self) {
    }
}

impl LifecycleListener for Window {
    fn on_update(&mut self) -> CrResult<()> {
        let action_time  = conrod_crayon::events::convert_event(&mut self.ui);
        let time_to_sleep:std::time::Duration = std::time::Duration::new(15, 0);
        if let Some(at) = action_time{
            self.action_instant = at;
        }
        {
            //let mut ui = self.ui.set_widgets();
            
            const LOGO_SIDE: conrod_core::Scalar = 306.0;
            self.game_process.run(&mut self.ui,
                                  &self.cardmeta,
                                  &mut self.game_data,
                                  &self.result_map);
            for s in network::receive(){
                self.game_process.update_state(&mut self.game_data, &self.result_map, s);
            }
            
        }
        let screen_dim = crayon::window::dimensions();
        let (screen_w,screen_h) = (screen_dim.x,screen_dim.y);
        let dpi_factor = crayon::window::device_pixel_ratio() as f64;
        //let dpi_factor  =1.16;
        if self.action_instant.elapsed() <= time_to_sleep {
            let primitives = self.ui.draw();
            let dims = (screen_w as f64 * dpi_factor, screen_h as f64 * dpi_factor);
            self.renderer.fill(dims,dpi_factor as f64,primitives,&self.image_map);
            self.renderer.draw(&mut self.batch,&self.image_map);
            /*
            opengl::draw_mutliple(self.batch,
                                &vertex_buffer,
                                &indices,
                                &program,
                                self.gamedata.page_vec,
                                self.result_map);*/
        }
        Ok(())
    }
}

main!({
    #[cfg(not(target_arch = "wasm32"))]
    let res = format!("file://{}/resources/", env!("CARGO_MANIFEST_DIR").replace("\\","/"));
    #[cfg(target_arch = "wasm32")]
    let res = format!("http://localhost:8080/resources/");
    let mut params = Params::default();
    params.window.title = "CR: RenderTexture".into();
    params.window.size = (WIN_W as u32, WIN_H as u32).into();
    params.res.shortcuts.add("res:", res).unwrap();
    params.res.dirs.push("res:".into());
    crayon::application::setup(params,|| {
        let resources = WindowResources::new()?;
        Ok(Launcher::new(resources, |r| Window::build(r)))
    }).unwrap();
});
