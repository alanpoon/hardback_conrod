extern crate hardback_conrod;
extern crate conrod_core;
extern crate conrod_crayon;
extern crate conrod_chat;
extern crate rodio;
extern crate crayon;
extern crate crayon_bytes;
extern crate instant;
#[macro_use]
extern crate cardgame_macros;
#[allow(non_snake_case)]
//
use conrod_core::text::{FontCollection};

use hardback_conrod as game_conrod;
use game_conrod::{app, logic};
use game_conrod::backend::{SupportIdType};
use game_conrod::backend::meta::app::{Font, ResourceEnum, AppData, MusicEnum,Sprite};
use game_conrod::backend::codec_lib::codec;
use game_conrod::page_curl::{self, page, render};
use game_conrod::opengl;
use game_conrod::on_request;
use game_conrod::support;
use game_conrod::backend::codec_lib;
use game_conrod::backend::codec_lib::cards;
use game_conrod::backend::WindowResources;
use game_conrod::app::BoardStruct;
use conrod_crayon::Renderer;
use crayon::prelude::*;
use crayon_bytes::prelude::*;
use crayon::network;
use std::collections::HashMap;
use instant::Instant;

#[derive(Clone)]
pub enum ConrodMessage {
    Event(Instant, conrod_core::event::Input),
    Thread(Instant),
}

const WIN_W: u32 = 1040;
const WIN_H: u32 = 542;

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
    p_surface: SurfaceHandle,
    p_shader: ShaderHandle
}
//crayon_bytes = { git = "https://github.com/alanpoon/crayon.git", branch ="textedit"}
//crayon = { git = "https://github.com/alanpoon/crayon.git", branch ="textedit"}

impl Window {
    pub fn build(resources: &WindowResources) -> CrResult<Self> {
        let screen_dim = crayon::window::dimensions();
        let (screen_w,screen_h) = (screen_dim.x,screen_dim.y);
        let dpi_factor = crayon::window::device_pixel_ratio();
        let appdata = AppData::new(screen_w as f64, screen_h as f64, "Hardback");
        let mut ui = conrod_core::UiBuilder::new([screen_w as f64 , screen_h as f64 ])
            .theme(support::theme(&appdata))
            .build();
        let mut result_map = HashMap::<ResourceEnum, SupportIdType>::new();
        let mut image_map = conrod_core::image::Map::new();
        resources.pump(&mut result_map,&mut image_map,&mut ui);
        if let Some(&SupportIdType::FontId(regular)) =
            result_map.get(&ResourceEnum::Font(Font::REGULAR)) {
            ui.theme.font_id = Some(regular);
        }
        if let Some(&SupportIdType::ImageId(th)) =
            result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS)) {
            
        }
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

        let p_attributes = AttributeLayoutBuilder::new()
            .with(Attribute::Position, 2)
            .with(Attribute::Texcoord0, 2)
            .finish();
        let p_uniforms = UniformVariableLayout::build()
            .with("scale", UniformVariableType::F32)
            .with("tex", UniformVariableType::Texture)
            .with("rotation", UniformVariableType::F32)
            .with("translation", UniformVariableType::F32)
            .with("theta", UniformVariableType::F32)
            .finish();
        let mut p_params = ShaderParams::default();
        //p_params.state.color_blend = Some((crayon::video::assets::shader::Equation::Add,
        //crayon::video::assets::shader::BlendFactor::Value(crayon::video::assets::shader::BlendValue::SourceAlpha),
        //crayon::video::assets::shader::BlendFactor::OneMinusValue(crayon::video::assets::shader::BlendValue::SourceAlpha)));
        p_params.attributes = p_attributes;
        p_params.uniforms = p_uniforms;
        let p_vs = include_str!("../page_curl/deform.vs").to_owned();;
        let p_fs = include_str!("../page_curl/deform.fs").to_owned();;
        let p_shader = video::create_shader(p_params.clone(), p_vs, p_fs).unwrap();
        let mut p_params = SurfaceParams::default();
        //p_params.set_clear(Color::gray(), None, None);
        let p_vert:Vec<page_curl::vertex::Vertex> = Vec::new();
        let p_surface = video::create_surface(p_params).unwrap();
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
            resources: *resources,
            p_surface: p_surface,
            p_shader: p_shader
        })
    }
}

impl Drop for Window {
    fn drop(&mut self) {
    }
}

impl LifecycleListener for Window {
    fn on_update(&mut self) -> CrResult<()> {
        let screen_dim = crayon::window::dimensions();
        let (screen_w,screen_h) = (screen_dim.x,screen_dim.y);
        self.ui.win_w = screen_w as f64;
        self.ui.win_h = screen_h as f64;
        let action_time  = conrod_crayon::events::convert_event(&mut self.ui);
        let time_to_sleep:std::time::Duration = std::time::Duration::new(15, 0);
        if let Some(at) = action_time{
            self.action_instant = at;
        }
        {
            
            const LOGO_SIDE: conrod_core::Scalar = 306.0;
            self.game_process.run(&mut self.ui,
                                  &mut self.image_map,
                                  &self.cardmeta,
                                  &mut self.game_data,
                                  &mut self.result_map
                                 );
            for s in network::receive(){
                self.game_process.update_state(&mut self.game_data, &self.result_map, s);
            }
            
        }
        let mut loaded = 0;
        
        opengl::draw_multiple(&mut self.batch,
                            &self.page.in_mesh,
                            &self.page.front_strip,
                            self.p_shader,
                            self.p_surface,
                            &mut self.game_data.page_vec,
                            &self.result_map);
                          
        let dpi_factor = crayon::window::device_pixel_ratio() as f64;
        let primitives = self.ui.draw();
        let dims = (screen_w as f64 * dpi_factor, screen_h as f64 * dpi_factor);
        //let dims = (screen_w as f64, screen_h as f64);
        self.renderer.fill(dims,dpi_factor as f64,primitives,&self.image_map);
        self.renderer.draw(&mut self.batch,&self.image_map);
        
        Ok(())
    }
}

main!({
    #[cfg(not(target_arch = "wasm32"))]
    let res = format!("file://{}/resources/", env!("CARGO_MANIFEST_DIR").replace("\\","/"));
    #[cfg(target_arch = "wasm32")]
    let res = format!("http://localhost:80/");
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
