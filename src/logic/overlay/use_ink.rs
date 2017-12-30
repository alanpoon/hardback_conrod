use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::full_cycle_sprite::FullCycleSprite;
use cardgame_widgets::custom_widget::tabview;
use cardgame_widgets::sprite::{SpriteInfo, spriteable_rect};
use backend::codec_lib::codec::*;
use backend::codec_lib;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{GameData, Ids, OverlayStatus, BoardStruct};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::{cards, local};
use graphics_match;
use logic::in_game;
use instruction::Instruction;
use custom_widget::show_draft_item;
pub fn render(w_id: tabview::Item,
              ids: &Ids,
              cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
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
                OverlayStatus::Received(card_index) => {
                    let (_timeless, _string, _color, _font, _rect) =
                        in_game::get_tile_image_withcost(card_index.clone(),
                                                         cardmeta,
                                                         appdata,
                                                         result_map);
                    if let (Some(&SupportIdType::ImageId(cloudy)),
                            Some(&SupportIdType::ImageId(coin_info)),
                            Some(&SupportIdType::ImageId(coin_info270))) =
                        (result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
                         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
                         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270))) {
                        show_draft_item::ItemWidget::new(_timeless, _string, _rect, "timeless")
                            .wh(appdata.convert_dim([150.0, 190.0]))
                            .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                            .cloudy_image(cloudy)
                            .coin_info(coin_info)
                            .coin_info270(coin_info270)
                            .alphabet_font_id(_font)
                            .color(_color)
                            .set(ids.overlay_receivedimage, ui);
                    }
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
