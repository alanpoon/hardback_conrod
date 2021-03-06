#[cfg(feature="android")]
pub mod assets_android;
#[cfg(feature="android")]
pub use support::assets_android as assets;
use backend::meta::app::AppData;
use conrod_core;
use std;
pub fn theme(appdata: &AppData) -> conrod_core::Theme {
    use conrod_core::position::{Align, Direction, Padding, Position, Relative};
    use conrod_core::FontSize;
    conrod_core::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::WHITE,
        border_color: conrod_core::color::BLACK,
        border_width: 2.0,
        label_color: conrod_core::color::BLACK,
        font_id: None,
        font_size_large: appdata.convert_h(30.0).floor() as FontSize,
        font_size_medium: appdata.convert_h(22.0).floor() as FontSize,
        font_size_small: appdata.convert_h(16.0).floor() as FontSize,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}
use conrod_core::{color, widget, Colorable, Sizeable, Positionable, Widget};
use std::collections::HashMap;
use backend::SupportIdType;
use backend::meta::app::ResourceEnum;
#[cfg(any(feature="android"))]
pub fn textedit(k: &mut String,
                id: widget::Id,
                _appdata: &AppData,
                result_map: &HashMap<ResourceEnum, SupportIdType>,
                dim: [f64; 2],
                limited_to_chars:Option<usize>,
                keypad_on_: &mut bool,
                left_of: widget::Id,
                right_margin: [f64;2],
                parent_of: widget::Id,
                ui: &mut conrod_core::UiCell) {
    use conrod_chat::chat::{english, sprite};
    use backend::meta::app::{AppData, ResourceEnum, Sprite};
    use conrod_chat::backend::custom_widget::text_edit::TextEdit;
    if let Some(&SupportIdType::ImageId(key_pad)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::KEYPAD)) {
        let english_tuple = english::populate(key_pad, sprite::get_spriteinfo());
        let (editz, keypad_bool) = TextEdit::new(k,parent_of,&english_tuple)
                    .color(color::BLACK)
                    .wh(dim)
                    .top_left_with_margins_on(left_of,right_margin[1],right_margin[0])
                    .left_justify()
                    .line_spacing(2.5)
                    .restrict_to_height(true) // Let the height grow infinitely and scroll.
                    .set(id, ui);
        for edit in editz {
            if let Some(_limited_to_chars)=limited_to_chars{
                let last_char = edit.chars().rev().take(_limited_to_chars).collect();             
                *k = last_char; 
            }else{
                *k = edit;
            }
        }
        *keypad_on_ = keypad_bool;
    }
}
#[cfg(feature="default")]
pub fn textedit(k: &mut String,
                id: widget::Id,
                _appdata: &AppData,
                _result_map: &HashMap<ResourceEnum, SupportIdType>,
                dim: [f64; 2],
                limited_to_chars:Option<usize>,
                _keypad_on_: &mut bool,
                left_of: widget::Id,
                right_margin: [f64;2],
                _parent_of: widget::Id,
                ui: &mut conrod_core::UiCell) {
    for edit in widget::TextEdit::new(k)
            .color(color::BLACK)
            .wh(dim)
            .top_left_with_margins_on(left_of,right_margin[1],right_margin[0])
            .left_justify()
            .line_spacing(2.5)
            .restrict_to_height(true) // Let the height grow infinitely and scroll.
            .set(id, ui) {
         if let Some(_limited_to_chars)=limited_to_chars{
                let last_char = edit.chars().rev().take(_limited_to_chars).collect();             
                *k = last_char; 
            }else{
                *k = edit;
            }
    }

}