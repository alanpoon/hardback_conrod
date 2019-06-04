use conrod_core::{self, color, widget, Colorable, Positionable, Widget, Sizeable, Labelable};
use std::collections::HashMap;
use futures::sync::mpsc;
use app::{GameData, Ids, GuiState};
use ui::RESULTMAPLEN;
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::custom_widget::progress_bar;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum};
use backend::codec_lib::codec::GameState;
#[allow(unused_mut)]
pub fn render(ui: &mut conrod_core::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    animated_canvas::Canvas::new().color(color::TRANSPARENT).frame_rate(30).set(ids.master, ui);
    let result_map_len = result_map.len();
    progress_bar::ProgressBar::new(result_map_len, RESULTMAPLEN)
        .middle_of(ids.master)
        .wh(appdata.convert_dim([300.0, 200.0]))
        .label("Loading")
        .set(ids.progress_bar, ui);
    if result_map_len >= RESULTMAPLEN {
        gamedata.guistate = GuiState::Game(GameState::ShowDraft);
    }
}
