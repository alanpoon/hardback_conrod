use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::sprite::spriteable_rect;
use cardgame_widgets::text::get_font_size_hn;
use custom_widget::show_draft_item;
use backend::codec_lib::codec::*;
use backend::codec_lib;
use std::collections::HashMap;
use app::{self, GameData, Ids, BoardStruct};
use graphics_match;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::{cards, local};
use logic::in_game;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    if let Some(card_index) = gamedata.overlay_blowup {
        if let (Some(&SupportIdType::ImageId(keypad_image)),
                Some(&SupportIdType::ImageId(cloudy)),
                Some(&SupportIdType::ImageId(coin_info)),
                Some(&SupportIdType::ImageId(coin_info270))) =
            (result_map.get(&ResourceEnum::Sprite(Sprite::KEYPAD)),
             result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
             result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
             result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270))) {

            let close_rect = spriteable_rect(graphics_match::keypad_sprite(), 2.0);
            if animated_canvas::Canvas::new()
                       .middle_of(ids.master)
                       .padded_wh_of(ids.master, appdata.convert_h(80.0))
                      /* .flow_down(&[(ids.overlaytop,
                                     animated_canvas::Canvas::new()
                                         .color(color::LIGHT_BLUE)
                                         .length(appdata.convert_h(100.0))),
                                    (ids.overlaybody,
                                     animated_canvas::Canvas::new()
                                         .color(color::LIGHT_BLUE))])
                       */
                       .color(color::LIGHT_BLUE)
                       .parent(ids.master)
                       .close_icon_color(color::YELLOW)
                       .close_icon_dim(appdata.convert_dim([30.0, 30.0]))
                       .close_icon(keypad_image)
                       .close_icon_src_rect(Rect::from_corners(close_rect.0, close_rect.1))
                       .frame_rate(30)
                       .set(ids.overlay, ui)
                       .is_done() {
                gamedata.overlay_blowup = None;
            }
            let (_timeless, _string, _color, _font, _rect, _genre_str, _non_genre_str) =
                in_game::get_tile_image_withcostwords(card_index.clone(),
                                                      cardmeta,
                                                      appdata,
                                                      result_map);
            let wh = ui.wh_of(ids.overlay).unwrap();

            show_draft_item::ItemWidget::new(_timeless, _string, _rect, "timeless")
                .cloudy_image(cloudy)
                .coin_info(coin_info)
                .coin_info270(coin_info270)
                .alphabet_font_id(_font)
                .color(_color)
                .w(wh[0] * 0.3)
                .h(wh[0] * 0.3 * 1.2)
                .mid_left_with_margin_on(ids.overlay, wh[0] * 0.05)
                .set(ids.blow_up_card, ui);
            let small_card_wh = [wh[0] * 0.05, wh[0] * 0.05 * 1.2];
            let font_size = get_font_size_hn(wh[1] * 0.3, 5.0);
            if let Some(_ng_str) = _non_genre_str {
                widget::Rectangle::fill_with(small_card_wh, _color)
                    .top_right_with_margins_on(ids.overlay, wh[1] * 0.1, wh[0] * 0.6)
                    .set(ids.blow_up_non_genre_rect, ui);
                widget::Image::new(cloudy)
                    .wh_of(ids.blow_up_non_genre_rect)
                    .middle_of(ids.blow_up_non_genre_rect)
                    .set(ids.blow_up_non_genre_cloudy, ui);
                widget::Text::new(&_ng_str)
                    .color(color::PURPLE.plain_contrast())
                    .font_size(font_size)
                    .top_right_with_margins_on(ids.overlay, wh[1] * 0.1, wh[0] * 0.45)
                    .w(wh[0] * 0.45)
                    .h(wh[1] * 0.3)
                    .set(ids.blow_up_non_genre_text, ui);
            }
            if let Some(_g_str) = _genre_str {
                widget::Rectangle::fill_with(small_card_wh, _color)
                    .top_right_with_margins_on(ids.overlay, wh[1] * 0.5, wh[0] * 0.6)
                    .set(ids.blow_up_genre_rect1, ui);
                widget::Image::new(cloudy)
                    .wh_of(ids.blow_up_genre_rect1)
                    .middle_of(ids.blow_up_genre_rect1)
                    .set(ids.blow_up_genre_cloudy1, ui);
                widget::Rectangle::fill_with(small_card_wh, _color)
                    .top_right_with_margins_on(ids.overlay, wh[1] * 0.5, wh[0] * 0.57)
                    .set(ids.blow_up_genre_rect2, ui);
                widget::Image::new(cloudy)
                    .wh_of(ids.blow_up_genre_rect2)
                    .middle_of(ids.blow_up_genre_rect2)
                    .set(ids.blow_up_genre_cloudy2, ui);
                widget::Text::new(&_g_str)
                    .color(color::PURPLE.plain_contrast())
                    .font_size(font_size)
                    .top_right_with_margins_on(ids.overlay, wh[1] * 0.5, wh[0] * 0.45)
                    .w(wh[0] * 0.45)
                    .h(wh[1] * 0.3)
                    .set(ids.blow_up_genre_text, ui);
            }
        }
    }
}
