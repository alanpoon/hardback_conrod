use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use cardgame_widgets::custom_widget::tabview;
use cardgame_widgets::custom_widget::full_cycle_sprite::FullCycleSprite;
use cardgame_widgets::custom_widget::image_panels::{Panelable, ImagePanels, ImageRectType};
use cardgame_widgets::sprite::SpriteInfo;
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
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
#[derive(Debug)]
pub struct Panel_Info<'a> {
    text: Option<String>,
    display_pic: Option<ImageRectType>,
    list_image: Vec<ImageRectType>,
    list_selected: &'a mut HashSet<usize, RandomState>,
}
impl<'b> Panelable for Panel_Info<'b> {
    fn text(&self) -> Option<String> {
        self.text.clone()
    }
    fn display_pic(&self) -> Option<ImageRectType> {
        self.display_pic
    }
    fn list_image(&self) -> Vec<ImageRectType> {
        self.list_image.clone()
    }
    fn list_selected<'a>(&'a self) -> &'a HashSet<usize, RandomState> {
        &self.list_selected
    }
    fn list_selected_mut<'a>(&'a mut self) -> &'a mut HashSet<usize, RandomState> {
        self.list_selected
    }
}
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
                   ref mut overlay_timeless_selected,
                   .. } = *gamedata;
    //normal_stuff don't need mut borrow
    let normal_stuff: Vec<(Option<String>, Option<ImageRectType>, Vec<ImageRectType>)> = vec![];
    let card_images = in_game::card_images(result_map);
    if let Some(&SupportIdType::ImageId(rust_logo)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::RUST)) {
        if let &mut Some(ref mut boardcodec) = boardcodec {
            for _p in boardcodec.players {
                let vec_cards = _p.timeless_classic
                    .iter()
                    .map(|x| {
                        let (_image_id, _rect, _) =
                            in_game::get_card_widget_image_flexible(x.clone(),
                                                                    &card_images,
                                                                    appdata);
                        let top_left = _rect.top_left();
                        let btm_right = _rect.bottom_right();
                        (_image_id, Some((top_left, btm_right)))
                    })
                    .collect::<Vec<ImageRectType>>();

                normal_stuff.push((Some(_p.name), Some((rust_logo, None)), vec_cards));
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
                        let spinner_sprite = graphics_match::spinner_sprite();
                        FullCycleSprite::new(dwn_img, spinner_sprite)
                            .mid_bottom_with_margin_on(ids.overlaybody, 20.0)
                            .w(100.0)
                            .h(100.0)
                            .set(ids.overlay_receivedimage, ui);
                    }
                }
                OverlayStatus::None => {
                    let mut vec_p = normal_stuff.iter()
                        .zip(overlay_timeless_selected.iter_mut())
                        .map(|(normal, list_selected)| {
                            Panel_Info {
                                text: normal.0.clone(),
                                display_pic: normal.1.clone(),
                                list_image: normal.2.clone(),
                                list_selected: list_selected,
                            }
                        })
                        .collect::<Vec<Panel_Info>>();
                    ImagePanels::new(&mut vec_p)
                        .middle_of(ids.overlaybody)
                        .padded_w_of(ids.overlaybody, 20.0)
                        .y_item_height(100.0)
                        .x_item_list([100.0, 100.0, 22.0, 5.0])
                        .set(ids.overlay_image_panels, ui);
                    for _c in widget::Button::new()
                            .label(&appdata.texts.use_timelessclassic)
                            .mid_bottom_with_margin_on(ids.overlaybody, 20.0)
                            .set(ids.overlay_okbut, ui) {
                        overlay_receivedimage[0] = OverlayStatus::Loading;
                        let action_tx_c = action_tx.clone();
                        let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
                        let mut g = GameCommand::new();
                        g.use_remover = Some(true);
                        h.set_gamecommand(g);
                        action_tx_c.send(OwnedMessage::Text(ServerReceivedMsg::serialize_send(h)
                                                                .unwrap()))
                            .wait()
                            .unwrap();
                    }

                }

            }
        }
    }
}
