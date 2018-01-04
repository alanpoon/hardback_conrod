use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::sprite::spriteable_rect;
use cardgame_widgets::text::get_font_size_hn;
use custom_widget::show_draft_item;
use custom_widget::blowup_detail;
use backend::codec_lib::codec::*;
use backend::codec_lib::cards::*;
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
                Some(&SupportIdType::ImageId(game_icon)),
                Some(&SupportIdType::ImageId(cloudy)),
                Some(&SupportIdType::ImageId(coin_info)),
                Some(&SupportIdType::ImageId(coin_info270))) =
            (result_map.get(&ResourceEnum::Sprite(Sprite::KEYPAD)),
             result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS)),
             result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
             result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
             result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270))) {

            let close_rect = spriteable_rect(graphics_match::keypad_sprite(), 2.0);
            if animated_canvas::Canvas::new()
                   .mid_top_with_margin_on(ids.master, appdata.convert_h(80.0))
                   .h(appdata.convert_h(80.0))
                   .padded_w_of(ids.master, appdata.convert_h(180.0))
                   .color(color::LIGHT_BLUE)
                   .parent(ids.master)
                   .close_icon_color(color::YELLOW)
                   .close_icon_dim(appdata.convert_dim([appdata.convert_h(40.0),
                                                        appdata.convert_h(40.0)]))
                   .close_icon(keypad_image)
                   .close_icon_src_rect(Rect::from_corners(close_rect.0, close_rect.1))
                   .frame_rate(30)
                   .set(ids.overlay, ui)
                   .is_done() {
                gamedata.overlay_blowup = None;
            }
            let (_timeless,
                 _string,
                 _color,
                 _font,
                 _rect,
                 _purchase,
                 _giveable,
                 _genre_giveable,
                 _trash,
                 _genre_str,
                 _non_genre_str) = in_game::get_tile_image_withcostwords(card_index.clone(),
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
                .set(ids.blowup_card, ui);
            let giveable_vec = vec![("Purchase".to_owned(),_purchase,None),
            ("One of a kind".to_owned(),_giveable,_non_genre_str),
            ("Two of a kind".to_owned(),_genre_giveable,_genre_str),
            ("Trash".to_owned(),_trash,None)];
            let reduce = giveable_vec.iter()
                .filter(|&&(ref _i, ref _g, ref _s)| if (_g != &GIVEABLE::NONE) | (_s.is_some()) {
                            true
                        } else {
                            false
                        })
                .collect::<Vec<&(String, GIVEABLE, Option<String>)>>();

            let (mut items, _) = widget::List::flow_down(reduce.len())
                .item_size(wh[0] / (reduce.len() as f64))
                .w(wh[0] * 0.6)
                .h(wh[1])
                .set(ids.blowup_list, ui);
            let mut reduce_iter = reduce.iter();
            while let (Some(item), Some(_g)) = (items.next(ui), reduce_iter.next()) {
                let &&(ref _is, ref _g, ref _s) = _g;
                let j =
                    blowup_detail::ItemWidget::new(&_is, _g.clone(), _s.clone(), game_icon.clone());
                item.set(j, ui);
            }

        }
    }
}
