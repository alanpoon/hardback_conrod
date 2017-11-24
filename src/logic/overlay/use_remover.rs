use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use cardgame_widgets::custom_widget::tabview;
use cardgame_widgets::custom_widget::full_cycle_sprite::FullCycleSprite;
use cardgame_widgets::sprite::SpriteInfo;
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
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>,
              ui: &mut conrod::UiCell) {
    let GameData { ref mut boardcodec,
                   ref player_index,
                   ref mut personal,
                   ref mut overlay_receivedimage,
                   .. } = *gamedata;
    //choose from the inked cards

    if let (&mut Some(ref mut boardcodec), &Some(ref _player_index)) = (boardcodec, player_index) {
        if let Some(ref _player) = boardcodec.players.get(_player_index.clone()) {
            let arranged = _player.arranged.clone();
            let inked = arranged.iter()
                .filter(|&&(_ci, _inked, _optstr)| _inked)
                .map(|&(_ci, _, _)| _ci.clone())
                .collect::<Vec<usize>>();
            let item_h = 230.0;
            let (mut events, scrollbar) = widget::ListSelect::single(inked.len())
                .flow_down()
                .item_size(item_h)
                .scrollbar_next_to()
                .w_h(700.0, 260.0)
                .middle_of(ids.overlaybody)
                .set(ids.overlay_explainlistselect, ui);
            let card_images = in_game::card_images(result_map);
            let mut card_index_sel = None;
            // Handle the `ListSelect`s events.
            while let Some(event) = events.next(ui, |i| if let Some(_ci) = card_index_sel {
                if _ci == inked.get(i).unwrap() {
                    true
                } else {
                    false
                }
            } else {
                false
            }) {
                use conrod::widget::list_select::Event;
                match event {
                    // For the `Item` events we instantiate the `List`'s items.
                    Event::Item(item) => {
                        let card_index = inked.get(item.i).unwrap();
                        let color = if let Some(_ci) = card_index_sel {
                            if _ci == inked.get(item.i).unwrap() {
                                conrod::color::YELLOW
                            } else {
                                conrod::color::LIGHT_GREY
                            }
                        } else {
                            conrod::color::LIGHT_GREY
                        };
                        let (_image_id, _rect, _) =
                            in_game::get_card_widget_image_portrait(card_index.clone(),
                                                                    &card_images,
                                                                    appdata);
                        let button =
                            widget::Button::image(_image_id).source_rectangle(_rect).color(color);
                        item.set(button, ui);
                    }

                    // The selection has changed.
                    Event::Selection(idx) => card_index_sel = Some(inked.get(idx).unwrap()),

                    // The remaining events indicate interactions with the `ListSelect` widget.
                    event => println!("{:?}", &event),
                }
            }

            // Instantiate the scrollbar for the list.
            if let Some(s) = scrollbar {
                s.set(ui);
            }
        }
    }

    if let (&mut Some(ref mut boardcodec), &Some(ref _player_index)) = (boardcodec, player_index) {
        if let Some(_player) = boardcodec.players.get_mut(_player_index.clone()) {

            match overlay_receivedimage[1] {
                OverlayStatus::Received(ref _img, ref _rect, ref _theme) => {
                    widget::Image::new(_img.clone())
                        .source_rectangle(_rect.clone())
                        .w(150.0)
                        .h(150.0)
                        .mid_bottom_with_margin_on(ids.overlaybody, 20.0)
                        .set(ids.overlay_receivedimage, ui);
                }
                OverlayStatus::Loading => {
                    if let Some(&SupportIdType::ImageId(dwn_img)) =
                        result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)) {
                        let spin_info = get_spinner_spriteinfo();
                        FullCycleSprite::new(dwn_img, spin_info)
                            .mid_bottom_with_margin_on(ids.overlaybody, 20.0)
                            .w(100.0)
                            .h(100.0)
                            .set(ids.overlay_receivedimage, ui);
                    }
                }
                OverlayStatus::None => {
                    if _player.remover > 0 {
                        for _c in widget::Button::new()
                                .label(&appdata.texts.use_remover)
                                .mid_bottom_with_margin_on(ids.overlaybody, 20.0)
                                .set(ids.overlay_okbut, ui) {
                            overlay_receivedimage[0] = OverlayStatus::Loading;
                            let action_tx_c = action_tx.clone();
                            let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
                            let mut g = GameCommand::new();
                            g.use_remover = Some(true);
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
