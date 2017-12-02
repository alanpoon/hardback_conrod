use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::custom_widget::tabview;
use cardgame_widgets::sprite::{SpriteInfo, spriteable_rect};
use cardgame_widgets::custom_widget::list_select::ListSelect;
use custom_widget::player_info::ItemWidget;
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids, Personal};
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
                       .pad(50.0)
                       .flow_down(&[(ids.overlaytop,
                                     animated_canvas::Canvas::new()
                                         .color(color::BLUE)
                                         .length(80.0)),
                                    (ids.overlaybody,
                                     animated_canvas::Canvas::new().color(color::BLUE))])
                       .color(color::LIGHT_BLUE)
                       .watch_state(gamedata.guistate.clone())
                       .close_icon(keypad_image)
                       .close_icon_src_rect(Rect::from_corners(close_rect.0, close_rect.1))
                       .frame_rate(30)
                       .set(ids.overlay, ui)
                       .is_done() {
                    gamedata.overlay = false;
                }
                let icon_v = graphics_match::gameicons_listitem(icon_image,
                                                                _player.ink.clone(),
                                                                _player.remover.clone(),
                                                                _player.coin.clone(),
                                                                _player.literacy_award.clone(),
                                                                _player.vp.clone(),
                                                                _player.draftlen.clone());
                ItemWidget::new(icon_v, &mut gamedata.overlay2).set(ids.overlay_player_info, ui);
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
                  &mut GameData,
                  &AppData,
                  &HashMap<ResourceEnum, SupportIdType>,
                  mpsc::Sender<OwnedMessage>,
                  &mut conrod::UiCell)>>
{
    vec![Box::new(|w_id, ids, mut gamedata, appdata, result_map, action_tx, ui| {
        //draw use ink
        overlay::use_ink::render(w_id,
                                 ids,
                                 &mut gamedata,
                                 &appdata,
                                 result_map,
                                 action_tx,
                                 ui);
    }),
         Box::new(|w_id, ids, mut gamedata, _appdata, result_map, action_tx, ui| {
        //draw use remover
        overlay::use_remover::render(w_id,
                                     ids,
                                     &mut gamedata,
                                     _appdata,
                                     result_map,
                                     action_tx,
                                     ui);
    }),
         Box::new(|w_id, ids, mut gamedata, _appdata, result_map, action_tx, ui| {
        //draw use timeless classics
        overlay::use_timelessclassic::render(w_id,
                                             ids,
                                             &mut gamedata,
                                             _appdata,
                                             result_map,
                                             action_tx,
                                             ui);
    })]
}
