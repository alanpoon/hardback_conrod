use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use cardgame_widgets::custom_widget::tabview;
use cardgame_widgets::custom_widget::full_cycle_sprite::FullCycleSprite;
use cardgame_widgets::custom_widget::bordered_image::BorderedImage;
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
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>,
              ui: &mut conrod::UiCell) {
    let GameData { ref mut boardcodec,
                   ref player_index,
                   ref mut overlay_receivedimage,
                   ref mut overlay_remover_selected,
                   .. } = *gamedata;
    //choose from the inked cards

    if let (&mut Some(ref mut boardcodec), &Some(ref _player_index)) = (boardcodec, player_index) {
        if let Some(ref _player) = boardcodec.players.get(_player_index.clone()) {
            let arranged = _player.arranged.clone();
            let inked = arranged.iter()
                .filter(|&&(ref _ci, ref _inked, ref _optstr, _)| _inked.clone())
                .map(|&(_ci, _, _, _)| _ci.clone())
                .collect::<Vec<usize>>();
            let item_h = 230.0;
            let (mut events, scrollbar) = widget::ListSelect::multiple(inked.len())
                .flow_down()
                .item_size(item_h)
                .scrollbar_next_to()
                .w_h(700.0, 260.0)
                .middle_of(w_id.parent_id)
                .set(ids.overlay_explainlistselect, ui);
            let card_images = in_game::card_images(result_map);
            // Handle the `ListSelect`s events.
            while let Some(event) = events.next(ui, |i| overlay_remover_selected.contains(&i)) {
                use conrod::widget::list_select::Event;
                match event {
                    // For the `Item` events we instantiate the `List`'s items.
                    Event::Item(item) => {
                        let card_index = inked.get(item.i).unwrap();
                        let selected = overlay_remover_selected.contains(&item.i);
                        let (_image_id, _rect, _) =
                            in_game::get_card_widget_image_portrait(card_index.clone(),
                                                                    &card_images,
                                                                    appdata);
                        let mut button = BorderedImage::new(_image_id)
                            .source_rectangle(_rect)
                            .border_color(color::YELLOW)
                            .border(20.0);
                        if selected {
                            button = button.bordered();
                        }
                        item.set(button, ui);
                    }

                    // The selection has changed.
                    Event::Selection(selection) => {
                        if overlay_remover_selected.len() < _player.remover {
                            selection.update_index_set(overlay_remover_selected);
                        }
                    }

                    // The remaining events indicate interactions with the `ListSelect` widget.
                    event => println!("{:?}", &event),
                }
            }

            // Instantiate the scrollbar for the list.
            if let Some(s) = scrollbar {
                s.set(ui);
            }

            match overlay_receivedimage[1] {
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
                        let spinner_sprite = graphics_match::spinner_sprite();
                        FullCycleSprite::new(dwn_img, spinner_sprite)
                            .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                            .w(100.0)
                            .h(100.0)
                            .set(ids.overlay_receivedimage, ui);
                    }
                }
                OverlayStatus::None => {
                    if overlay_remover_selected.len() > 0 {
                        for _c in widget::Button::new()
                                .label(&appdata.texts.use_remover)
                                .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                                .set(ids.overlay_okbut, ui) {
                            overlay_receivedimage[0] = OverlayStatus::Loading;
                            let action_tx_c = action_tx.clone();
                            let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
                            let mut g = GameCommand::new();
                            let selected_vec = overlay_remover_selected.iter()
                                .map(|x| inked.get(x.clone()).unwrap().clone())
                                .collect::<Vec<usize>>();
                            g.use_remover = Some(selected_vec);
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