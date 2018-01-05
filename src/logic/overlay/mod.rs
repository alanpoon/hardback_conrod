use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::custom_widget::tabview;
use cardgame_widgets::sprite::{SpriteInfo, spriteable_rect};
use conrod::widget::primitive::image::Image;
use cardgame_widgets::custom_widget::player_info::list::List;
use cardgame_widgets::custom_widget::player_info::item::IconStruct;
use cardgame_widgets::text::get_font_size_hn;
use backend::codec_lib::codec::*;
use backend::codec_lib;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids, BoardStruct};
use graphics_match;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::{cards, local};
use logic::{overlay, in_game};
use instruction::Instruction;
pub mod use_ink;
pub mod use_remover;
pub mod use_timelessclassic;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {
    if let (Some(boardcodec), Some(player_index)) =
        (gamedata.boardcodec.clone(), gamedata.player_index) {
        if let (Some(_player), true) = (boardcodec.players.get(player_index), gamedata.overlay) {
            if let (Some(&SupportIdType::ImageId(keypad_image)),
                    Some(&SupportIdType::ImageId(icon_image))) =
                (result_map.get(&ResourceEnum::Sprite(Sprite::KEYPAD)),
                 result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
                let close_rect = spriteable_rect(graphics_match::keypad_sprite(), 2.0);
                if animated_canvas::Canvas::new()
                       .middle_of(ids.master)
                       .padded_wh_of(ids.master, appdata.convert_h(30.0))
                       .flow_down(&[(ids.overlaytop,
                                     animated_canvas::Canvas::new()
                                         .color(color::LIGHT_BLUE)
                                         .length(appdata.convert_h(100.0))),
                                    (ids.overlaybody,
                                     animated_canvas::Canvas::new()
                                         .color(color::LIGHT_BLUE))])
                       .color(color::TRANSPARENT)
                       .parent(ids.master)
                       .close_icon_color(color::WHITE)
                       .close_icon_dim(appdata.convert_dim([30.0, 30.0]))
                       .close_icon(keypad_image)
                       .close_icon_src_rect(Rect::from_corners(close_rect.0, close_rect.1))
                       .frame_rate(30)
                       .set(ids.overlay, ui)
                       .is_done() {
                    gamedata.overlay = false;
                }
                let default_color = color::GREY;
                let icon_v = graphics_match::gameicons_listitem(icon_image,
                                                                _player.ink.clone(),
                                                                _player.remover.clone(),
                                                                _player.coin.clone(),
                                                                _player.literacy_award.clone(),
                                                                _player.vp.clone(),
                                                                _player.draftlen.clone());

                let slist = List::new(icon_v.clone(), &mut gamedata.overlay2)
                    .color(default_color)
                    .label("Player Info")
                    .label_color(default_color.plain_contrast())
                    .wh_of(ids.overlaytop)
                    .middle_of(ids.overlaytop)
                    .set(ids.overlay_player_info, ui);

                if let (Some(_s), Some(_si), Some(xy)) = slist {
                    let _dim = appdata.convert_dim([300.0, 100.0]);
                    animated_canvas::Canvas::new()
                        .x(appdata.convert_w(xy[0]))
                        .y(appdata.convert_h(200.0))
                        .graphics_for(ids.master)
                        .parent(ids.master)
                        .color(default_color)
                        .wh(_dim)
                        .set(ids.overlay2_canvas, ui);
                    if let Some(&IconStruct(ref _image, _, ref _desc)) = icon_v.get(_s) {
                        _image.wh(appdata.convert_dim([20.0, 20.0]))
                            .mid_left_of(ids.overlay2_canvas)
                            .set(ids.overlay2_image, ui);
                        let fontsize = get_font_size_hn(_dim[1], 4.0);
                        widget::Text::new(&_desc)
                            .font_size(fontsize)
                            .color(default_color.plain_contrast())
                            .align_middle_y_of(ids.overlay2_image)
                            .right_from(ids.overlay2_image, 0.0)
                            .w(_dim[0] - 20.0)
                            .h(_dim[1])
                            .set(ids.overlay2_text, ui);
                    }

                }

            }

            if let Some(mut items) = tabview::TabView::new(vec![appdata.texts.use_ink,
                                                            appdata.texts.use_remover,
                                                            appdata.texts.use_timelessclassic])
                       .padded_wh_of(ids.overlaybody, 0.0)
                       .mid_top_of(ids.overlaybody)
                       .set(ids.overlaybody_tabview, ui) {
                let vec_closure = render_closure();
                let mut it_j = vec_closure.iter();
                while let (Some(a), Some(item)) = (it_j.next(), items.next(ui)) {
                    let action_tx_clone = action_tx.clone();
                    (*a)(item,
                         ids,
                         cardmeta,
                         gamedata,
                         appdata,
                         result_map,
                         action_tx_clone,
                         ui);
                }

            }

        }
    }

}
fn render_closure()
    -> Vec<Box<Fn(tabview::Item,
                  &Ids,
                  &[codec_lib::cards::ListCard<BoardStruct>; 180],
                  &mut GameData,
                  &AppData,
                  &HashMap<ResourceEnum, SupportIdType>,
                  mpsc::Sender<OwnedMessage>,
                  &mut conrod::UiCell)>>
{
    vec![Box::new(|w_id, ids, cardmeta, mut gamedata, appdata, result_map, action_tx, ui| {
        //draw use ink
        overlay::use_ink::render(w_id,
                                 ids,
                                 &cardmeta,
                                 &mut gamedata,
                                 &appdata,
                                 result_map,
                                 action_tx,
                                 ui);
    }),
         Box::new(|w_id, ids, cardmeta, mut gamedata, _appdata, result_map, action_tx, ui| {
        //draw use remover
        overlay::use_remover::render(w_id,
                                     ids,
                                     &cardmeta,
                                     &mut gamedata,
                                     _appdata,
                                     result_map,
                                     action_tx,
                                     ui);
    }),
         Box::new(|w_id, ids, cardmeta, mut gamedata, _appdata, result_map, action_tx, ui| {
        //draw use timeless classics
        overlay::use_timelessclassic::render(w_id,
                                             ids,
                                             &cardmeta,
                                             &mut gamedata,
                                             _appdata,
                                             result_map,
                                             action_tx,
                                             ui);
    })]
}
