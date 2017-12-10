use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::custom_widget::tabview;
use cardgame_widgets::sprite::{SpriteInfo, spriteable_rect};
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids};
use graphics_match;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use logic;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {
    if gamedata.overlay_chat {
        if let Some(&SupportIdType::ImageId(keypad_image)) =
            result_map.get(&ResourceEnum::Sprite(Sprite::KEYPAD)) {
            let close_rect = spriteable_rect(graphics_match::keypad_sprite(), 2.0);
            if animated_canvas::Canvas::new()
                   .middle_of(ids.master)
                   .padded_wh_of(ids.master, 20.0)
                   .flow_down(&[(ids.overlaybody_chat,
                                 animated_canvas::Canvas::new().color(color::LIGHT_BLUE))])
                   .color(color::TRANSPARENT)
                   .parent(ids.master)
                   .close_icon_dim([20.0, 20.0])
                   .close_icon(keypad_image)
                   .close_icon_src_rect(Rect::from_corners(close_rect.0, close_rect.1))
                   .frame_rate(30)
                   .set(ids.overlay_chat, ui)
                   .is_done() {
                gamedata.overlay_chat = false;
            }

        }

        if let Some(mut items) = tabview::TabView::new(vec![appdata.texts.chat])
               .padded_wh_of(ids.overlaybody_chat, 0.0)
               .mid_top_of(ids.overlaybody_chat)
               .set(ids.overlaybody_tabview_chat, ui) {
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
fn render_closure()
    -> Vec<Box<Fn(tabview::Item,
                  &Ids,
                  &mut GameData,
                  &AppData,
                  &HashMap<ResourceEnum, SupportIdType>,
                  mpsc::Sender<OwnedMessage>,
                  &mut conrod::UiCell)>>
{
    vec![
             Box::new(|w_id, ids, mut gamedata, _appdata, result_map, action_tx, ui| {
            //Chat
            logic::top_left::draw_lobby_chat(w_id, ids, &mut gamedata, result_map, action_tx, ui);
        })]
}
