use hardback_meta::app::{AppData, ResourceEnum};
use conrod::{self, color, widget, Colorable, Widget, text, Borderable};
use std::collections::HashMap;
use futures::sync::mpsc;
use backend::codec_lib::cards::*;
use backend::codec_lib::cards;
use logic;
use app::{GameData, Ids, GuiState, BoardStruct};
use ui::{Vala, load_resources_iter, iter_resource_enum_vala_next};
use backend::OwnedMessage;
use backend::SupportIdType;
use cardgame_widgets::custom_widget::animated_canvas;
use std::sync::mpsc::Sender;
use std;
use image;
pub struct GameProcess<'a, T>
    where T: Clone
{
    pub update_closure: Box<Fn(&mut GameData,
                               &AppData,
                               &HashMap<ResourceEnum, SupportIdType>,
                               T) + 'a>,
    pub appdata: AppData,
    pub ids: Ids,
}

impl<'a, T> GameProcess<'a, T>
    where T: Clone
{
    pub fn new(ui: &mut conrod::Ui,
               appdata: AppData,
               y: Box<Fn(&mut GameData,
                         &AppData,
                         &HashMap<ResourceEnum, SupportIdType>,
                         T) + 'a>)
               -> GameProcess<'a, T> {
        GameProcess {
            update_closure: y,
            appdata: appdata,
            ids: Ids::new(ui.widget_id_generator()),
        }
    }
    pub fn run(&mut self,
               ui: &mut conrod::Ui,
               cardmeta: &[cards::ListCard<BoardStruct>; 180],
               mut gamedata: &mut GameData,
               result_map: &HashMap<ResourceEnum, SupportIdType>,
               load_asset_tx: Sender<(ResourceEnum,
                                      Option<image::RgbaImage>,
                                      Option<text::Font>)>,
               action_tx: mpsc::Sender<OwnedMessage>) {
        let ids = &self.ids;
        // remove last_send if elasped 2 second
        if let Some(_last_send) = gamedata.last_send {
            if _last_send.elapsed() > std::time::Duration::new(3, 0) {
                gamedata.last_send = None;
            }
        }
        match &gamedata.guistate {
            &GuiState::Game(_) => {
                if result_map.len() < 20 {
                    gamedata.guistate = GuiState::Loading;
                    std::thread::spawn(move || {
                        let mut map: HashMap<ResourceEnum, Vala> = HashMap::new();
                        load_resources_iter(&mut map);
                        let mut _iter_resource_enum_vala = map.iter();
                        while let Some((k, v)) = _iter_resource_enum_vala.next() {
                            let _send_this = iter_resource_enum_vala_next((*k).clone(),
                                                                          (*v).clone());
                            load_asset_tx.send(_send_this).unwrap();
                        }
                    });
                } else {
                    self.set_game_ui(&mut ui.set_widgets(),
                                     &ids,
                                     &mut gamedata,
                                     &self.appdata,
                                     &cardmeta,
                                     result_map,
                                     action_tx);
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
                                     result_map,
                                     action_tx);
            }
            &GuiState::Loading => {
                logic::loading::render(&mut ui.set_widgets(),
                                       &ids,
                                       &mut gamedata,
                                       &self.appdata,
                                       result_map);
            }
            _ => {}
        }
        //  logic::notification::render(&mut ui.set_widgets(), &ids, gamedata.notification.clone());
    }
    fn set_game_ui(&self,
                   ui: &mut conrod::UiCell,
                   ids: &Ids,
                   mut gamedata: &mut GameData,
                   appdata: &AppData,
                   cardmeta: &[cards::ListCard<BoardStruct>; 180],
                   result_map: &HashMap<ResourceEnum, SupportIdType>,
                   action_tx: mpsc::Sender<OwnedMessage>) {

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
                            result_map,
                            action_tx.clone());

        logic::footer::render(ui,
                              ids,
                              &mut gamedata,
                              &appdata,
                              &cardmeta,
                              result_map,
                              action_tx.clone());
        logic::overlay_blowup::render(ui, ids, &cardmeta, &mut gamedata, &appdata, result_map);
        logic::overlay::render(ui,
                               ids,
                               &cardmeta,
                               &mut gamedata,
                               &appdata,
                               result_map,
                               action_tx.clone());
        logic::overlay_chat::render(ui,
                                    ids,
                                    &mut gamedata,
                                    &appdata,
                                    result_map,
                                    action_tx.clone());
        logic::overlay_exit::render(ui,
                                    ids,
                                    &mut gamedata,
                                    &appdata,
                                    result_map,
                                    action_tx.clone());
        logic::overlay_prompt::render(ui, ids, &mut gamedata, action_tx.clone());
    }
    #[allow(unused_mut)]
    pub fn update_state(&self,
                        mut gamedata: &mut GameData,
                        result_map: &HashMap<ResourceEnum, SupportIdType>,
                        msg: T) {
        (*self.update_closure)(gamedata, &self.appdata, result_map, msg);
    }
}
