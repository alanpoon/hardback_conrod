use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use conrod::position::rect::Rect;
use conrod::widget::primitive::image::Image;
use conrod::widget::envelope_editor::EnvelopePoint;
use cardgame_widgets::custom_widget::sample_drag_image;
use cardgame_widgets::custom_widget::image_hover::{Hoverable, ImageHover};
use cardgame_widgets::custom_widget::shuffle::Shuffle;
use cardgame_widgets::custom_widget::dragdrop_list::DragDropList;
use cardgame_widgets::custom_widget::promptview::{PromptView, PromptSender};
use backend::codec_lib::codec::*;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};

use app::{self, GameData, Ids, Personal, GuiState};
use logic::in_game;
use graphics_match;
pub struct ImageHoverable(Image, Option<Image>, Option<Image>);
impl Hoverable for ImageHoverable {
    fn idle(&self) -> Image {
        self.0
    }
    fn hover(&self) -> Option<Image> {
        self.1
    }
    fn press(&self) -> Option<Image> {
        self.2
    }
}
#[derive(Clone)]
pub struct PromptSendable(mpsc::Sender<OwnedMessage>);
impl PromptSender for PromptSendable {
    fn send(&self, msg: String) {
        self.0
            .clone()
            .send(OwnedMessage::Text(msg))
            .wait()
            .unwrap();
    }
}
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              _action_tx: mpsc::Sender<OwnedMessage>) {
    let GameData { ref page_index,
                   ref mut boardcodec,
                   ref mut print_instruction_set,
                   ref mut guistate,
                   ref mut initial_draft,
                   ref player_index,
                   ref mut personal,
                   .. } = *gamedata;
    if let &mut Some(ref mut boardcodec) = boardcodec {
        let card_images = in_game::card_images(result_map);
        if let Some(ref mut _player) = boardcodec.players.get_mut(*page_index) {
            match guistate {
                &mut app::GuiState::Game(GameState::ShowDraft) => {
                    show_draft(ui,
                               ids,
                               _player,
                               &card_images,
                               &appdata,
                               print_instruction_set,
                               initial_draft,
                               result_map,
                               _action_tx.clone());
                }
                &mut app::GuiState::Game(GameState::Shuffle) => {
                    shuffle(ui,
                            ids,
                            _player,
                            &card_images,
                            &appdata,
                            &initial_draft,
                            player_index,
                            guistate,
                            result_map);
                }
                &mut app::GuiState::Game(GameState::Spell) => {
                    cache_personal(_player, personal);
                    spell(ui, ids, &card_images, personal, appdata, result_map);
                }
                &mut app::GuiState::Game(GameState::TurnToSubmit) => {
                    spell(ui, ids, &card_images, personal, appdata, result_map);
                    turn_to_submit_but(ui, ids, &appdata);
                }
                _ => {}
            }
        }
    }

    //  draw_hand(ui, ids, gamedata, appdata, result_map);
}
fn turn_to_submit_but(ui: &mut conrod::UiCell, ids: &Ids, appdata: &AppData) {
    widget::Button::new()
        .label(&appdata.texts.submit)
        .mid_bottom_of(ids.body)
        .w_h(100.0, 80.0)
        .set(ids.submit_but, ui);
}
fn show_draft(ui: &mut conrod::UiCell,
              ids: &Ids,
              player: &mut Player,
              card_images: &[Option<image::Id>; 27],
              appdata: &AppData,
              print_instruction_set: &mut Vec<bool>,
              initial_draft: &mut Vec<usize>,
              _result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {
    let body_w = ui.w_of(ids.body).unwrap();
    let item_h = body_w / 5.0;
    *initial_draft = player.draft.clone();
    let mut dealt_iter = player.draft.iter();
    if let Some(&mut true) = print_instruction_set.get_mut(0) {
        let (mut items, scrollbar) = widget::List::flow_right(player.draft.len())
            .item_size(item_h)
            .instantiate_all_items()
            .mid_bottom_of(ids.body)
            .h(item_h * 1.2)
            .padded_w_of(ids.body, 20.0)
            .scrollbar_next_to()
            .set(ids.listview, ui);
        if let Some(s) = scrollbar {
            s.set(ui)
        }
        while let (Some(item), Some(card_index)) = (items.next(ui), dealt_iter.next()) {
            let (_image_id, _rect, _) =
                in_game::get_card_widget_image_portrait(card_index.clone(), card_images, appdata);
            //zoom rect
            let mut top_left_c = _rect.top_left().clone();
            top_left_c.set_x(_rect.top_left().get_x() + 100.0);
            top_left_c.set_y(_rect.top_left().get_y() - 120.0);
            let btm_right = _rect.bottom_right().clone();
            let _zoom_rect = Rect::from_corners(top_left_c, btm_right);
            let _ih = ImageHoverable(Image::new(_image_id).source_rectangle(_rect),
                                     Some(Image::new(_image_id).source_rectangle(_zoom_rect)),
                                     None);
            let j = ImageHover::new(_ih);
            item.set(j, ui);
        }
    } else {

        let promptsender = PromptSendable(action_tx);
        let instructions: Vec<(&str, Box<Fn(PromptSendable)>)> = vec![("Continue",  Box::new(move|ps| {
                            let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
                            let mut g = GameCommand::new();
                            g.go_to_shuffle = Some(true);
                            h.set_gamecommand(g);
                            ps.send(ServerReceivedMsg::serialize_send(h).unwrap());
                        }))];
        let prompt_j = PromptView::new(&instructions,
                                       (0.5, "Lets' start to Shuffle the cards"),
                                       promptsender)
                .padded_wh_of(ids.body, 100.0)
                .middle_of(ids.body);
        prompt_j.set(ids.promptview, ui);
    }
}
fn shuffle(ui: &mut conrod::UiCell,
           ids: &Ids,
           player: &Player,
           card_images: &[Option<image::Id>; 27],
           appdata: &AppData,
           initial_draft: &Vec<usize>,
           player_index: &Option<usize>,
           guistate: &mut GuiState,
           _result_map: &HashMap<ResourceEnum, SupportIdType>) {
    if let (Some(&SupportIdType::ImageId(back_logo)), &Some(_player_index)) =
        (_result_map.get(&ResourceEnum::Sprite(Sprite::BACKCARD)), player_index) {
        let card_vec = initial_draft.iter()
            .map(|x| {
                     let (_image_id, _rect, _) =
                    in_game::get_card_widget_image_portrait(x.clone(), card_images, appdata);
                     Image::new(_image_id).source_rectangle(_rect)
                 })
            .collect::<Vec<Image>>();
        let give_out_vec = player.hand
            .iter()
            .enumerate()
            .map(move |(_index, x)| {
                let mut _z = None;
                for ref _d in initial_draft.clone() {
                    if _d == x {
                        _z = Some(_index);
                    }
                }
                _z
            })
            .filter(|x| if let &Some(_) = x { true } else { false })
            .map(|x| x.unwrap())
            .collect::<Vec<usize>>();
        println!("give_out_vec {:?}", give_out_vec);
        let back_rect = Rect::from_corners([670.0, 70.0], [1130.0, 850.0]);
        if !Shuffle::new(card_vec, Image::new(back_logo).source_rectangle(back_rect))
                .give_out(give_out_vec)
                .bottom_left_of(ids.body)
                .w(400.0)
                .close_frame_rate(25)
                .set(ids.shuffleview, ui) {
            if _player_index == 0 {
                *guistate = GuiState::Game(GameState::TurnToSubmit);
            } else {
                *guistate = GuiState::Game(GameState::Spell);
            }
        }
    }
}
fn cache_personal(player: &mut Player, personal: &mut Option<Personal>) {
    if let &mut None = personal {
        *personal = Some(Personal {
                             hand: player.hand.clone(),
                             arranged: vec![],
                         });
    }
}
fn spell(ui: &mut conrod::UiCell,
         ids: &Ids,
         card_images: &[Option<image::Id>; 27],
         personal: &mut Option<Personal>,
         appdata: &AppData,
         result_map: &HashMap<ResourceEnum, SupportIdType>) {
    if let &mut Some(ref mut _personal) = personal {
        let mut arrangedvec =
            _personal.arranged
                .clone()
                .iter()
                .map(|&(ref x, ref ink, ref op_string, ref _timeless)| {
                    let (_image_id, _rect, _) =
                        in_game::get_card_widget_image_portrait(x.clone(), card_images, appdata);
                    (x.clone(), _image_id, _rect, ink.clone(), op_string.clone(), _timeless.clone())
                })
                .collect::<Vec<(usize, image::Id, conrod::Rect, bool, Option<String>, bool)>>();
        if let (Some(&SupportIdType::ImageId(spinner_image)),
                Some(&SupportIdType::ImageId(rust_image))) =
            (result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
             result_map.get(&ResourceEnum::Sprite(Sprite::RUST))) {
            let spinner_rect = graphics_match::spinner_sprite();
            let exitid =
                DragDropList::new(&mut arrangedvec,
                                  Box::new(move |(_v_index, v_blowup, v_rect, _, _, _)| {
                    sample_drag_image::Button::image(v_blowup)
                        .source_rectangle(v_rect)
                        .toggle_image(rust_image.clone())
                        .spinner_image(spinner_image, spinner_rect)
                        .w_h(200.0, 230.0)
                }),
                                  50.0)
                        .h(260.0)
                        .padded_w_of(ids.body, 20.0)
                        .mid_bottom_with_margin_on(ids.body, 80.0)
                        .exit_id(Some(Some(ids.footerdragdroplistview)))
                        .set(ids.bodydragdroplistview, ui);
            if let Some((v_index, _, _, _, _, _)) = exitid {
                _personal.hand.push(v_index);
            }
            _personal.arranged = arrangedvec.iter()
                .map(|&(ref x_index, _, _, ref ink, ref op_string, ref timeless)| {
                         (x_index.clone(), ink.clone(), op_string.clone(), timeless.clone())
                     })
                .collect::<Vec<(usize, bool, Option<String>, bool)>>();
        }
    }

}
