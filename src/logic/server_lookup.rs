use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, Labelable};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::Instant;
use futures::sync::mpsc;
use app::{GameData, Ids, GuiState};
use cardgame_widgets::custom_widget::animated_canvas;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum};
use support;
use cardgame_widgets::custom_widget::notification::Notification;
#[allow(unused_mut)]
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              server_lookup_tx: Sender<String>,
              notification: Option<(String, Instant)>
              ) {
    animated_canvas::Canvas::new().color(color::TRANSPARENT).frame_rate(30).set(ids.master, ui);
    //
    let wh = ui.wh_of(ids.master).unwrap();
    widget::Text::new(&gamedata.name)
                    .color(color::WHITE)
                    .right_from(ids.name_text, 0.0)
                    .w_h(appdata.convert_w(200.0), appdata.convert_h(wh[1] * 0.06))
                    .set(ids.user_name, ui);
    widget::Rectangle::fill_with([appdata.convert_w(200.0), wh[1] * 0.06],
                                    color::WHITE)
            .right_from(ids.user_name, 0.0)
            .set(ids.name_rect, ui);
    support::textedit(&mut gamedata.server_lookup,
                    ids.name_text_edit,
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
    if let Some((s, i)) = notification {
        Notification::new(&s, i)
            .top_right_of(ids.body)
            .color(color::GREY)
            .wh([240.0, 80.0])
            .set(ids.notification_view, ui);
    }
}
