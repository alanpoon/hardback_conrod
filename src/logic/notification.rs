use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable};
use cardgame_widgets::custom_widget::notification::Notification;
use app::Ids;
use std::time::Instant;
pub fn render(ui: &mut conrod::UiCell, ids: &Ids, notification: Option<(String, Instant)>) {
    if let Some((s, i)) = notification {
        Notification::new(&s, i)
            .top_right_of(ids.body)
            .color(color::GREY)
            .wh([180.0, 80.0])
            .set(ids.notification_view, ui);
    }

}
