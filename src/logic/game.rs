use hardback_meta::app::{AppData, ResourceEnum};
use conrod::{self, color, widget, Colorable, Widget};
use std::collections::HashMap;
use futures::sync::mpsc;
use logic;
use app::{GameData, Ids, GuiState};
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
               y: Box<Fn(&mut GameData,
                         &AppData,
                         &HashMap<ResourceEnum, SupportIdType>,
                         T) + 'a>)
               -> GameProcess<'a, T> {
        let appdata = AppData::new(1200, 800, "Hardback");
        GameProcess {
            update_closure: y,
            appdata: appdata,
            ids: Ids::new(ui.widget_id_generator()),
        }
    }
    pub fn run(&mut self,
               ui: &mut conrod::Ui,
               mut gamedata: &mut GameData,
               result_map: &HashMap<ResourceEnum, SupportIdType>,
               action_tx: mpsc::Sender<OwnedMessage>) {
        //    let mut ids = Ids::new(ui.widget_id_generator());
        let ids = &self.ids;
        match &gamedata.guistate {
            &GuiState::Game(_) => {
                self.set_game_ui(&mut ui.set_widgets(),
                                 &ids,
                                 &mut gamedata,
                                 &self.appdata,
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
                   result_map: &HashMap<ResourceEnum, SupportIdType>,
                   action_tx: mpsc::Sender<OwnedMessage>) {

        animated_canvas::Canvas::new()
            .color(color::TRANSPARENT)
            .flow_down(&[(ids.body, animated_canvas::Canvas::new().color(color::TRANSPARENT)),
                         (ids.footer,
                          animated_canvas::Canvas::new().color(color::DARK_GREEN).length(300.0))])
            .watch_state(gamedata.guistate.clone())
            // .close_icon(rust_logo)
            .frame_rate(30)
            .set(ids.master, ui);
        logic::body::render(ui,
                            ids,
                            &mut gamedata,
                            &appdata,
                            result_map,
                            action_tx.clone());

        logic::footer::render(ui,
                              ids,
                              &mut gamedata,
                              &appdata,
                              result_map,
                              action_tx.clone());
        logic::overlay::render(ui,
                               ids,
                               &mut gamedata,
                               &appdata,
                               result_map,
                               action_tx.clone());
    }
    #[allow(unused_mut)]
    pub fn update_state(&self,
                        mut gamedata: &mut GameData,
                        result_map: &HashMap<ResourceEnum, SupportIdType>,
                        msg: T) {
        (*self.update_closure)(gamedata, &self.appdata, result_map, msg);
    }
}
