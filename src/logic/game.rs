use hardback_meta::app::{AppData, ResourceEnum, Font, Sprite};
use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use conrod::widget::list::{Right, Dynamic};
use std::collections::HashMap;
use std;
use futures::sync::mpsc;
use futures::{Future, Sink};
use logic;
use app::{self, GameData, Ids, GameState};
use backend::OwnedMessage;
use backend::SupportIdType;
const LIB_PATH: &'static str = "target/debug/libtest_shared.so";

pub struct GameProcess<'a, T>
    where T: Clone
{
    pub update_closure: Box<Fn(&mut GameData, &HashMap<ResourceEnum, SupportIdType>, T) + 'a>,
    pub appdata: AppData,
    pub ids: Ids,
}

impl<'a, T> GameProcess<'a, T>
    where T: Clone
{
    pub fn new(ui: &mut conrod::Ui,
               y: Box<Fn(&mut GameData, &HashMap<ResourceEnum, SupportIdType>, T) + 'a>)
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
        match &gamedata.gamestate {
            &GameState::Start => {
                self.set_game_ui(&mut ui.set_widgets(),
                                 &ids,
                                 &mut gamedata,
                                 &self.appdata,
                                 result_map,
                                 action_tx);
            }
            &GameState::Menu => {

                logic::menu::render(&mut ui.set_widgets(),
                                    &ids,
                                    &mut gamedata,
                                    &self.appdata,
                                    result_map,
                                    action_tx);
            }
            &GameState::Lobby => {
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
                   mut ui: &mut conrod::UiCell,
                   ids: &Ids,
                   mut gamedata: &mut GameData,
                   appdata: &AppData,
                   result_map: &HashMap<ResourceEnum, SupportIdType>,
                   action_tx: mpsc::Sender<OwnedMessage>) {

        widget::Canvas::new()
            .color(color::TRANSPARENT)
            .flow_down(&[(ids.body, widget::Canvas::new().color(color::TRANSPARENT)),
                         (ids.footer,
                          widget::Canvas::new().color(color::DARK_GREEN).length(100.0))])
            .set(ids.master, ui);
        logic::footer::render(ui,
                              ids,
                              &mut gamedata,
                              &appdata,
                              result_map,
                              action_tx.clone());

    }
    pub fn update_state(&self,
                        mut gamedata: &mut GameData,
                        result_map: &HashMap<ResourceEnum, SupportIdType>,
                        msg: T) {
        (*self.update_closure)(gamedata, result_map, msg);
    }
}
