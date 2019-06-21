use hardback_meta::app::{AppData, ResourceEnum,Texture,Sprite};
use conrod_core::{self, color, widget, Colorable, Widget, text, Borderable};
use std::collections::HashMap;
use backend::codec_lib::cards::*;
use backend::codec_lib::cards;
use backend::SupportIdType;
use logic;
use app::{GameData, Ids, GuiState, BoardStruct};
use ui::{Vala, RESULTMAPLEN};
use cardgame_widgets::custom_widget::animated_canvas;
use crayon::prelude::*;
use crayon_audio::prelude::*;
use crayon_bytes::prelude::*;
use std::sync::mpsc::Sender;
use std;
use image;
pub struct GameProcess<T>
    where T: Clone
{
    pub update_closure: Box<Fn(&mut GameData,
                               &AppData,
                               &HashMap<ResourceEnum, SupportIdType>,
                               T)>,
    pub appdata: AppData,
    pub ids: Ids,
}

impl<T> GameProcess<T>
    where T: Clone
{
    pub fn new(ids: Ids,
               appdata: AppData,
               y: Box<Fn(&mut GameData,
                         &AppData,
                         &HashMap<ResourceEnum, SupportIdType>,
                         T)>)
               -> GameProcess<T> {
        GameProcess {
            update_closure: y,
            appdata: appdata,
            ids: ids,//Ids::new(ui.widget_id_generator()),
        }
    }
    pub fn run(&mut self,
               ui: &mut conrod_core::Ui,
               image_map: &mut conrod_core::image::Map<TextureHandle>,
               cardmeta: &[cards::ListCard<BoardStruct>; 180],
               mut gamedata: &mut GameData,
               result_map: &mut HashMap<ResourceEnum, SupportIdType>) {
        let ids = &self.ids;
        // remove last_send if elasped 2 second
        if let Some(_last_send) = gamedata.last_send {
            if _last_send.elapsed() > std::time::Duration::new(3, 0) {
                gamedata.last_send = None;
            }
        }
        match &gamedata.guistate {
            &GuiState::Game(_) => {
                if result_map.len() < RESULTMAPLEN {
                    gamedata.guistate = GuiState::Loading;
                    result_map.insert(ResourceEnum::Texture(Texture::PAGE1F),SupportIdType::TextureId(impl_value!{
                        "texture","res:player1.jpg"
                    }));
                    result_map.insert(ResourceEnum::Texture(Texture::PAGE2F),SupportIdType::TextureId(impl_value!{
                        "texture","res:player2.jpg"
                    }));
                    result_map.insert(ResourceEnum::Texture(Texture::PAGE3F),SupportIdType::TextureId(impl_value!{
                        "texture","res:player3.jpg"
                    }));
                    result_map.insert(ResourceEnum::Texture(Texture::PAGE4F),SupportIdType::TextureId(impl_value!{
                        "texture","res:player4.jpg"
                    }));
                    result_map.insert(ResourceEnum::Sprite(Sprite::DOWNLOAD),SupportIdType::TextureId(impl_value!{
                        "texture","res:download.png"
                    }));
                    result_map.insert(ResourceEnum::Sprite(Sprite::BACKCARD),SupportIdType::TextureId(impl_value!{
                        "texture","res:backside.jpg"
                    }));
                    result_map.insert(ResourceEnum::Sprite(Sprite::ARROWS),SupportIdType::TextureId(impl_value!{
                        "texture","res:arrows_but.png"
                    }));
                } else {
                    
                    self.set_game_ui(&mut ui.set_widgets(),
                                     &ids,
                                     &mut gamedata,
                                     &self.appdata,
                                     &cardmeta,
                                     result_map);
                }
            }
            &GuiState::Menu => {
                logic::menu::render(&mut ui.set_widgets(),
                                    &ids,
                                    &mut gamedata,
                                    &self.appdata,
                                    &cardmeta,
                                    result_map);
            }
            &GuiState::Lobby => {
                
                logic::lobby::render(&mut ui.set_widgets(),
                                     &ids,
                                     &mut gamedata,
                                     &self.appdata,
                                     result_map);
            }
            &GuiState::Loading => {
                logic::loading::render(&mut ui.set_widgets(),
                                       &ids,
                                       &mut gamedata,
                                       &self.appdata,
                                       result_map,
                                       image_map);
            }
            _ => {}
        }
        //  logic::notification::render(&mut ui.set_widgets(), &ids, gamedata.notification.clone());
    }
    fn set_game_ui(&self,
                   ui: &mut conrod_core::UiCell,
                   ids: &Ids,
                   mut gamedata: &mut GameData,
                   appdata: &AppData,
                   cardmeta: &[cards::ListCard<BoardStruct>; 180],
                   result_map: &HashMap<ResourceEnum, SupportIdType>) {

        animated_canvas::Canvas::new()
            .pad_top(appdata.convert_h(40.0))
            .color(color::TRANSPARENT)
            .flow_down(&[(ids.body, animated_canvas::Canvas::new().color(color::TRANSPARENT)),
                         (ids.footer,
                          animated_canvas::Canvas::new()
                              .color(color::DARK_GREEN)
                              .length(appdata.convert_h(210.0)))])
            .frame_rate(30)
            .border(0.0)
            .set(ids.master, ui);
           
        logic::body::render(ui,
                            ids,
                            &mut gamedata,
                            &appdata,
                            &cardmeta,
                            result_map);
                         
        logic::footer::render(ui,
                              ids,
                              &mut gamedata,
                              &appdata,
                              &cardmeta,
                              result_map);
        
        logic::overlay_blowup::render(ui, ids, &cardmeta, &mut gamedata, &appdata, result_map);
        logic::overlay::render(ui,
                               ids,
                               &cardmeta,
                               &mut gamedata,
                               &appdata,
                               result_map);
        logic::overlay_chat::render(ui,
                                    ids,
                                    &mut gamedata,
                                    &appdata,
                                    result_map);
        logic::overlay_exit::render(ui,
                                    ids,
                                    &mut gamedata,
                                    &appdata,
                                    result_map);
        logic::overlay_human::render(ui, ids, &mut gamedata, &appdata, result_map);
        logic::overlay_prompt::render(ui, ids, &mut gamedata);
        
    }
    #[allow(unused_mut)]
    pub fn update_state(&self,
                        mut gamedata: &mut GameData,
                        result_map: &HashMap<ResourceEnum, SupportIdType>,
                        msg: T) {
        (*self.update_closure)(gamedata, &self.appdata, result_map, msg);
    }
}
