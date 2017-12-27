use hardback_meta::app::{AppData, ResourceEnum};
use conrod::{self, color, widget, Colorable, Widget};
use std::collections::HashMap;
use futures::sync::mpsc;
use backend::codec_lib::cards::*;
use backend::codec_lib::cards;
use logic;
use app::{GameData, Ids, GuiState, BoardStruct};
use backend::OwnedMessage;
use backend::SupportIdType;
use cardgame_widgets::custom_widget::animated_canvas;
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
               action_tx: mpsc::Sender<OwnedMessage>) {
        let ids = &self.ids;
        match &gamedata.guistate {
            &GuiState::Game(_) => {
                self.set_game_ui(&mut ui.set_widgets(),
                                 &ids,
                                 &mut gamedata,
                                 &self.appdata,
                                 &cardmeta,
                                 result_map,
                                 action_tx);

            }
            &GuiState::Menu => {
                logic::menu::render(&mut ui.set_widgets(),
                                    &ids,
                                    &mut gamedata,
                                    &self.appdata,
                                    result_map,
                                    action_tx);
            }
            &GuiState::Lobby => {
                logic::lobby::render(&mut ui.set_widgets(),
                                     &ids,
                                     &mut gamedata,
                                     &self.appdata,
                                     result_map,
                                     action_tx);
            }
            _ => {}
        }
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
        logic::overlay::render(ui,
                               ids,
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
