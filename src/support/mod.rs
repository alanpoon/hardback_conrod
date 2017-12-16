#[cfg(target_os="android")]
pub mod assets_android;
#[cfg(target_os="android")]
pub use support::assets_android as assets;
#[cfg(target_os="linux")]
pub mod assets;
use backend::meta::app::AppData;
use conrod;
use std;
pub fn theme(appdata: &AppData) -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    use conrod::FontSize;
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: appdata.convert_h(26.0).floor() as FontSize,
        font_size_medium: appdata.convert_h(18.0).floor() as FontSize,
        font_size_small: appdata.convert_h(12.0).floor() as FontSize,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}
use conrod::{color, widget, Colorable, Sizeable, Positionable, Widget};
use std::collections::HashMap;
use backend::SupportIdType;
use backend::meta::app::ResourceEnum;
#[cfg(not(target_os="linux"))]
pub fn textedit(k: &mut String,
                id: widget::Id,
                appdata: &AppData,
                result_map: &HashMap<ResourceEnum, SupportIdType>,
                dim: [f64; 2],
                keypad_on_: &mut bool,
                right_of: widget::Id,
                right_margin: f64,
                parent_of: widget::Id,
                ui: &mut conrod::UiCell) {
    use conrod_chat::chat::{english, sprite};
    use conrod_keypad::custom_widget::text_edit::TextEdit;
    if let Some(&SupportIdType::ImageId(key_pad)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::KEYPAD)) {
        let english_tuple = english::populate(key_pad, sprite::get_spriteinfo());
        for (editz, keypad_on) in
            TextEdit::new(k,parent_of)
                    .color(color::BLACK)
                    .wh(dim)
                    .right_from(right_of,right_margin)
                    .left_justify()
                    .line_spacing(2.5)
                    .restrict_to_height(true) // Let the height grow infinitely and scroll.
                    .set(id, ui) {
            *k = editz;
        }
        keypad_on_ = keypad_on;
    }

}
#[cfg(target_os="linux")]
pub fn textedit(k: &mut String,
                id: widget::Id,
                _appdata: &AppData,
                _result_map: &HashMap<ResourceEnum, SupportIdType>,
                dim: [f64; 2],
                _keypad_on_: &mut bool,
                right_of: widget::Id,
                right_margin: f64,
                _parent_of: widget::Id,
                ui: &mut conrod::UiCell) {
    for editz in widget::TextEdit::new(k)
            .color(color::BLACK)
            .wh(dim)
            .right_from(right_of,right_margin)
            .left_justify()
            .line_spacing(2.5)
            .restrict_to_height(true) // Let the height grow infinitely and scroll.
            .set(id, ui) {
        *k = editz;
    }

}
