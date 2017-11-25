use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::full_cycle_sprite::FullCycleSprite;
use cardgame_widgets::custom_widget::tabview;
use cardgame_widgets::sprite::{SpriteInfo, spriteable_rect};
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{GameData, Ids, Personal, OverlayStatus};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::{cards, local};
use graphics_match;
use logic::in_game;
use instruction::Instruction;
pub fn render(w_id: tabview::Item,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>,
              ui: &mut conrod::UiCell) {
    let GameData { ref mut boardcodec, ref player_index, ref mut overlay_receivedimage, .. } =
        *gamedata;
    //explanation pictures
    let (mut items, _) = widget::List::flow_right(3)
        .item_size(220.0)
        .middle_of(w_id.parent_id)
        .w_of(w_id.parent_id)
        .set(ids.overlay_explainlist, ui);
    if let (Some(&SupportIdType::ImageId(back_image)),
            Some(&SupportIdType::ImageId(icon_image))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::BACKCARD)),
         result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
        let game_icon_sprite = graphics_match::gameicon_sprite();
        let back_rect = Rect::from_corners([670.0, 70.0], [1130.0, 850.0]);
        let arrow_rect = spriteable_rect(game_icon_sprite, 4.0);
        let question_rect = spriteable_rect(game_icon_sprite, 5.0);
        let image_vec =
            vec![(back_image, back_rect),
                             (icon_image, Rect::from_corners(arrow_rect.0, arrow_rect.1)),
                             (icon_image, Rect::from_corners(question_rect.0, question_rect.1))];
        let mut image_iter = image_vec.iter();
        while let (Some(item), Some(&(imageid, imagerect))) = (items.next(ui), image_iter.next()) {
            let j = widget::Image::new(imageid).source_rectangle(imagerect);
            item.set(j, ui);
        }
    }
    if let (&mut Some(ref mut boardcodec), &Some(ref _player_index)) = (boardcodec, player_index) {
        if let Some(_player) = boardcodec.players.get_mut(_player_index.clone()) {
            match overlay_receivedimage[0] {
                OverlayStatus::Received(ref _img, ref _rect, ref _theme) => {
                    widget::Image::new(_img.clone())
                        .source_rectangle(_rect.clone())
                        .w(150.0)
                        .h(150.0)
                        .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                        .set(ids.overlay_receivedimage, ui);
                }
                OverlayStatus::Loading => {
                    if let Some(&SupportIdType::ImageId(dwn_img)) =
                        result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)) {
                        let spin_sprite = graphics_match::spinner_sprite();
                        FullCycleSprite::new(dwn_img, spin_sprite)
                            .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                            .w(100.0)
                            .h(100.0)
                            .set(ids.overlay_receivedimage, ui);
                    }
                }
                OverlayStatus::None => {
                    if _player.ink > 0 {
                        for _c in widget::Button::new()
                                .label(&appdata.texts.use_ink)
                                .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                                .set(ids.overlay_okbut, ui) {
                            overlay_receivedimage[0] = OverlayStatus::Loading;
                            let action_tx_c = action_tx.clone();
                            let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
                            let mut g = GameCommand::new();
                            g.take_card_use_ink = Some(true);
                            h.set_gamecommand(g);
                            action_tx_c.send(OwnedMessage::Text(ServerReceivedMsg::serialize_send(h).unwrap()))
                            .wait()
                            .unwrap();
                        }
                    }
                }

            }
        }
    }
}
