use conrod_core::{self, color, widget, Colorable, Positionable, Widget, Sizeable};
use cardgame_widgets::custom_widget::notification::Notification;
use app::Ids;
use std::time::Instant;
pub fn render(ui: &mut conrod_core::UiCell,
              ids: &Ids,
              top_right_of: widget::Id,
              notification: Option<(String, Instant)>) {
    if let Some((s, i)) = notification {
        Notification::new(&s, i)
            .top_right_of(top_right_of)
            .color(color::GREY)
            .wh([240.0, 80.0])
            .set(ids.notification_view, ui);
    }

}
