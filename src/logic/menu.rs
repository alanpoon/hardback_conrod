use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use conrod::widget::list_select::Event;
use conrod::widget::list::Right;
use conrod::widget::list::Fixed;
use cardgame_widgets::custom_widget::animated_button;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids, GameState};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Font, Sprite};
use graphics_match::button;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {
    widget::Canvas::new().color(color::TRANSPARENT).set(ids.master, ui);
    if widget::Button::new()
           .w_h(200.0, 50.0)
           .mid_left_of(ids.master)
           .rgb(0.4, 0.75, 0.6)
           .label("Multiplayer")
           .set(ids.menubut_multiplayer, ui)
           .was_clicked() {
        gamedata.gamestate = GameState::Lobby;
    }
}
