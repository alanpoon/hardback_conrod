use conrod_core::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable,
             Borderable};
use cardgame_widgets::custom_widget::tabview;
use cardgame_widgets::custom_widget::full_cycle_sprite::FullCycleSprite;
use cardgame_widgets::custom_widget::bordered_image::BorderedImage;
use cardgame_widgets::custom_widget::bordered_image::Bordered;
use custom_widget::buy_list_item;
use custom_widget::show_draft_item;
use cardgame_widgets::sprite::SpriteInfo;
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
pub fn render(w_id: tabview::Item,
              ids: &Ids,
              cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>,
              ui: &mut conrod_core::UiCell) {
    let GameData { ref mut boardcodec,
                   ref player_index,
                   ref mut overlay_receivedimage,
                   ref mut overlay_remover_selected,
                   .. } = *gamedata;
    //choose from the inked cards
    widget::Text::new(&appdata.texts.use_remover)
        .color(color::WHITE)
        .font_size(60)
        .h(appdata.convert_h(100.0))
        .w_of(w_id.parent_id)
        .top_left_of(w_id.parent_id)
        .set(ids.overlay_subject, ui);
    if let (&mut Some(ref mut boardcodec), &Some(ref _player_index)) = (boardcodec, player_index) {
        if let Some(ref _player) = boardcodec.players.get(_player_index.clone()) {
            let arranged = _player.arranged.clone();
            let inked = arranged.iter()
                .filter(|&&(ref _ci, ref _inked, ref _optstr, _)| _inked.clone())
                .map(|&(_ci, _, _, _)| _ci.clone())
                .collect::<Vec<usize>>();
            let item_h = appdata.convert_h(230.0);
            let (mut events, scrollbar) = widget::ListSelect::multiple(inked.len())
                .flow_down()
                .item_size(item_h)
                .scrollbar_next_to()
                .w_of(w_id.parent_id)
                .h(appdata.convert_h(260.0))
                .down_from(ids.overlay_subject, 0.0)
                .set(ids.overlay_explainlistselect, ui);
            if let (Some(&SupportIdType::ImageId(cloudy)),
                    Some(&SupportIdType::ImageId(coin_info)),
                    Some(&SupportIdType::ImageId(coin_info270)),
                    Some(&SupportIdType::ImageId(dwn_img)),
                    Some(&SupportIdType::ImageId(_game_icon))) =
                (result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
                 result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
                 result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270)),
                 result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
                 result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
                // Handle the `ListSelect`s events.
                while let Some(event) = events.next(ui, |i| overlay_remover_selected.contains(&i)) {
                    use conrod_core::widget::list_select::Event;
                    match event {
                        // For the `Item` events we instantiate the `List`'s items.
                        Event::Item(item) => {
                            let card_index = inked.get(item.i).unwrap();
                            let selected = overlay_remover_selected.contains(&item.i);
                            let (_timeless, _string, _color, _font, _rect, _top_left_rect) =
                                in_game::get_tile_image_withcost(card_index.clone(),
                                                                 cardmeta,
                                                                 appdata,
                                                                 result_map);

                            let mut j = buy_list_item::ItemWidget::new(_timeless,
                                                                       _string,
                                                                       _rect,
                                                                       _top_left_rect,
                                                                       "timeless")
                                    .game_icon(_game_icon)
                                    .wh(appdata.convert_dim([150.0, 190.0]))
                                    .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                                    .cloudy_image(cloudy)
                                    .coin_info(coin_info)
                                    .coin_info270(coin_info270)
                                    .alphabet_font_id(_font)
                                    .border_color(color::YELLOW)
                                    .border(20.0)
                                    .color(_color);
                            if selected {
                                j = j.bordered();
                            }
                            item.set(j, ui);

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
                    OverlayStatus::Received(ref card_index) => {
                        let (_timeless, _string, _color, _font, _rect, _top_left_rect) =
                            in_game::get_tile_image_withcost(card_index.clone(),
                                                             cardmeta,
                                                             appdata,
                                                             result_map);
                        show_draft_item::ItemWidget::new(_timeless,
                                                         _string,
                                                         _rect,
                                                         _top_left_rect,
                                                         "timeless")
                                .wh(appdata.convert_dim([150.0, 190.0]))
                                .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                                .cloudy_image(cloudy)
                                .coin_info(coin_info)
                                .coin_info270(coin_info270)
                                .alphabet_font_id(_font)
                                .color(_color)
                                .set(ids.overlay_receivedimage, ui);
                    }
                    OverlayStatus::Loading => {
                        let spinner_sprite = graphics_match::spinner_sprite();
                        FullCycleSprite::new(dwn_img, spinner_sprite)
                            .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                            .wh(appdata.convert_dim([150.0, 150.0]))
                            .set(ids.overlay_receivedimage, ui);
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

}
