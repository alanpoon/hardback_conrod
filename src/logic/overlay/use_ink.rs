use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use cardgame_widgets::custom_widget::full_cycle_sprite;
use cardgame_widgets::sprite::SpriteInfo;
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{GameData, Ids, Personal, OverlayStatus};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::{cards, gameicon};
use backend::meta::local;
use logic::in_game;
use instruction::Instruction;
pub fn render(ui: &mut conrod::UiCell,
              w_id: tabview::Item,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {
    let GameData { ref mut boardcodec,
                   ref player_index,
                   ref mut personal,
                   ref mut overlay_receivedimage,
                   .. } = *gamedata;
    //explanation pictures
    let (mut items, _) = widget::List::flow_right(3)
        .item_size(220.0)
        .middle_of(ids.overlaybody)
        .w_of(ids.overlaybody)
        .set(ids.overlay_explainlist, ui);
    if let Some(&SupportIdType::ImageId(icon_image)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS)) {

        let back_rect = Rect::from_corners([670.0, 70.0], [1130.0, 850.0]);
        let arrow_rect = gameicon::sprite().src_rect(4.0);
        let question_rect = gameicon::sprite().src_rect(5.0);
        let image_vec =
            vec![(back_image,back_rect),(icon_image,arrow_rect),(icon_image,question_rect)];
        let mut image_iter = image_vec.iter();
        while let (Some(item), Some((imageid, imagerect))) = (items.next(ui), image_iter.next()) {
            let j = widget::Image::new(imageid).source_rectangle(imagerect);
            item.set(j, ui);
        }
    }
    if let &mut Some(ref mut boardcodec) = boardcodec {
        if let Some(ref mut _player) = boardcodec.players.get_mut(*player_index) {
            match overlay_receivedimage[0] {
                &mut OverlayStatus::Received(ref _img, ref _rect, ref _theme) => {
                    widget::Image::new(_img)
                        .source_rectangle(_rect.clone())
                        .w(150.0)
                        .h(150.0)
                        .mid_bottom_with_margins(ids.overlaybody, 20.0)
                        .set(ids.overlay_receivedimage, ui);
                }
                &mut OverlayStatus::Loading => {
                    if let Some(&SupportIdType::ImageId(dwn_img)) =
                        result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)) {
                        let spin_info = get_spinner_spriteinfo();
                        FullCycleSprite::new(dwn_img, spin_info)
                            .mid_bottom_with_margins(ids.overlaybody, 20.0)
                            .w(100.0)
                            .h(100.0)
                            .set(ids.overlay_receivedimage, ui);
                    }
                }
                &mut OverlayStatus::None => {
                    if _player.ink > 0 {
                        for _c in widget::Button::new()
                                .label(&appdata.texts.use_ink_but)
                                .mid_bottom_with_margins(ids.overlaybody, 20.0)
                                .set(ids.overlay_okbut, ui) {
                            *overlay_receivedimage[0] = OverlayStatus::Loading;
                            let action_tx_c = _action_tx.clone();
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
fn get_spinner_spriteinfo() -> SpriteInfo {
    SpriteInfo {
        first: (0.0, 400.0),
        num_in_row: 12,
        num_in_col: 4,
        w_h: (100.0, 100.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
}
