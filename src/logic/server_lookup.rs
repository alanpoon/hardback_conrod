use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, Labelable};
use std::collections::HashMap;
use futures::sync::mpsc;
use app::{GameData, Ids, GuiState};
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::custom_widget::progress_bar;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum};
use backend::codec_lib::codec::GameState;
#[allow(unused_mut)]
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              server_lookup_tx: Sender<String>
              ) {
    animated_canvas::Canvas::new().color(color::TRANSPARENT).frame_rate(30).set(ids.master, ui);
    support::textedit(&mut gamedata.server_lookup,
                    ids.server_lookup_text_edit,
                    appdata,
                    result_map,
                    [appdata.convert_w(195.0), wh[1] * 0.06],
                    None,
                    &mut gamedata.keypad_on,
                    ids.user_name,
                    wh[0] * 0.025,
                    ids.master,
                    ui);
    for _i in widget::Button::new()
        .label(appdata.texts.connect)
        .label_font_size(14)
        .label_color(color::BLACK)
        .right_from(ids.name_rect, 2.0)
        .w_h(wh[0] * 0.3, wh[1] * 0.06)
        .set(ids.submit_but, ui) {
            server_lookup_tx.send(gamedata.server_lookup.clone()).unwrap();
        }
    widget::Text::new(gamedata.server_lookup_txt)
            .bottom_right_with_margin_on(ids.master, 50.0)
            .color(color::WHITE)
            .font_size(40)
            .set(ids.menu_version_num, ui);
}
