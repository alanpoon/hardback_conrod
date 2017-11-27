use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, Labelable};
use std::collections::HashMap;
use futures::sync::mpsc;
use app::{GameData, Ids, GuiState};
use cardgame_widgets::custom_widget::animated_canvas;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum};
#[allow(unused_mut)]
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              _appdata: &AppData,
              _result_map: &HashMap<ResourceEnum, SupportIdType>,
              _action_tx: mpsc::Sender<OwnedMessage>) {
    animated_canvas::Canvas::new()
        .color(color::TRANSPARENT)
        .watch_state(gamedata.guistate.clone())
        .frame_rate(30)
        .set(ids.master, ui);
    if widget::Button::new()
           .w_h(200.0, 50.0)
           .mid_left_of(ids.master)
           .rgb(0.4, 0.75, 0.6)
           .label("Multiplayer")
           .set(ids.menubut_multiplayer, ui)
           .was_clicked() {
        gamedata.guistate = GuiState::Lobby;
    }

}
