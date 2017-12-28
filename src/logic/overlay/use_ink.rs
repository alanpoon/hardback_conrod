use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::full_cycle_sprite::FullCycleSprite;
use cardgame_widgets::custom_widget::tabview;
use cardgame_widgets::sprite::{SpriteInfo, spriteable_rect};
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{GameData, Ids, OverlayStatus};
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
    widget::Text::new(&appdata.texts.use_ink)
        .color(color::WHITE)
        .font_size(60)
        .h(appdata.convert_h(100.0))
        .w_of(w_id.parent_id)
        .top_left_of(w_id.parent_id)
        .set(ids.overlay_subject, ui);
    if let (&mut Some(ref mut boardcodec), &Some(ref _player_index)) = (boardcodec, player_index) {
        if let Some(_player) = boardcodec.players.get_mut(_player_index.clone()) {
            match overlay_receivedimage[0] {
                OverlayStatus::Received(_cardindex) => {
                    /*widget::Image::new(_img.clone())
                        .source_rectangle(_rect.clone())
                        .wh(appdata.convert_dim([150.0, 150.0]))
                        .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                        .set(ids.overlay_receivedimage, ui);*/
                }
                OverlayStatus::Loading => {
                    if let Some(&SupportIdType::ImageId(dwn_img)) =
                        result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)) {
                        let spin_sprite = graphics_match::spinner_sprite();
                        FullCycleSprite::new(dwn_img, spin_sprite)
                            .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                            .wh(appdata.convert_dim([100.0, 100.0]))
                            .set(ids.overlay_receivedimage, ui);
                    }
                }
                OverlayStatus::None => {
                    if _player.ink > 0 {
                        for _c in widget::Button::new()
                                .label(&appdata.texts.use_ink)
                                .mid_bottom_with_margin_on(w_id.parent_id,
                                                           appdata.convert_h(20.0))
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
                    } else {
                        widget::Text::new(&appdata.texts.use_ink_insufficent)
                            .color(color::WHITE)
                            .font_size(24)
                            .padded_wh_of(w_id.parent_id, appdata.convert_h(40.0))
                            .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                            .set(ids.overlay_insufficent_text, ui);
                    }
                }

            }
        }
    }
}
