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
use backend::Message;
use backend::SupportIdType;
const LIB_PATH: &'static str = "target/debug/libtest_shared.so";
pub struct GameInstance<'a, T>
    where T: Clone
{
    pub update_closure: Box<Fn(&mut GameData,
                               &HashMap<ResourceEnum, SupportIdType>,
                               ConrodMessage<T>) + 'a>,
}

impl<'a, T> GameInstance<'a, T>
    where T: Clone
{
    pub fn new(y: Box<Fn(&mut GameData,
                         &HashMap<ResourceEnum, SupportIdType>,
                         ConrodMessage<T>) + 'a>)
               -> GameInstance<'a, T> {
        GameInstance { update_closure: y }
    }

    pub fn run(&self,
               ui: &mut conrod::Ui,
               mut gamedata: &mut GameData,
               result_map: &HashMap<ResourceEnum, SupportIdType>,
               event_rx: std::sync::mpsc::Receiver<ConrodMessage<T>>,
               action_tx: mpsc::Sender<Message>,
               render_tx: std::sync::mpsc::Sender<conrod::render::OwnedPrimitives>,
               events_loop_proxy: glium::glutin::EventsLoopProxy) {
        let mut ids = Ids::new(ui.widget_id_generator());
        //   ids.menu_buts.resize(3, &mut ui.widget_id_generator());
        //  let anim_num = gamedata.animationids.len();
        //  ids.animations.resize(anim_num, &mut ui.widget_id_generator());
        let mut needs_update = true;
        let mut last_update = std::time::Instant::now();
        let appdata = AppData::new(1200, 800, "Hardback");
        'conrod: loop {
            let sixteen_ms = std::time::Duration::from_millis(16);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }
            // Collect any pending events.
            let mut events = Vec::new();
            while let Ok(event) = event_rx.try_recv() {
                self.update_state(&mut gamedata, &result_map, event.clone());
                events.push(event);
            }
            if events.is_empty() || !needs_update {
                match event_rx.recv() {
                    Ok(event) => {
                        self.update_state(&mut gamedata, &result_map, event.clone());
                        events.push(event);
                    }
                    Err(_) => break 'conrod,
                };
            }

            needs_update = false;
            // Input each event into the `Ui`.
            for event in events {
                if let ConrodMessage::Event(e) = event {
                    ui.handle_event(e);
                }
                needs_update = true;
            }
            let action_tx = action_tx.clone();
            match &gamedata.gamestate {
                &GameState::Start => {
                    self.set_game_ui(&mut ui.set_widgets(),
                                     &ids,
                                     &mut gamedata,
                                     &appdata,
                                     result_map,
                                     action_tx);
                }
                _ => {}
            }

            if let Some(primitives) = ui.draw_if_changed() {
                if render_tx.send(primitives.owned()).is_err() ||
                   events_loop_proxy.wakeup().is_err() {
                    break 'conrod;
                }
            }
        }
    }
    fn update_state(&self,
                    mut gamedata: &mut GameData,
                    result_map: &HashMap<ResourceEnum, SupportIdType>,
                    conrod_msg: ConrodMessage<T>) {
        (*self.update_closure)(gamedata, result_map, conrod_msg);
    }
    fn set_game_ui(&self,
                   mut ui: &mut conrod::UiCell,
                   ids: &Ids,
                   mut gamedata: &mut GameData,
                   appdata: &AppData,
                   result_map: &HashMap<ResourceEnum, SupportIdType>,
                   action_tx: mpsc::Sender<Message>) {
        widget::Canvas::new()
            .color(color::TRANSPARENT)
            .flow_down(&[(ids.body, widget::Canvas::new()),
                         (ids.footer,
                          widget::Canvas::new().color(color::DARK_GREEN).length(100.0))])
            .set(ids.master, ui);

    }
}

#[derive(Clone,Debug)]
pub enum ConrodMessage<T: Clone> {
    Event(conrod::event::Input),
    Socket(T), 
  //  Animate(app::AnimateMsg),
}
