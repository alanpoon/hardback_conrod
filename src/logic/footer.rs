use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::dragdrop_list::DragDropList;
use cardgame_widgets::custom_widget::sample_drag_image;
use cardgame_widgets::custom_widget::instructionset::InstructionSet;
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::sprite::spriteable_rect;
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids, Personal};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::{cards, local};
use graphics_match;
use logic::in_game;
use instruction::Instruction;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              _action_tx: mpsc::Sender<OwnedMessage>) {
    let GameData { ref page_index,
                   ref mut boardcodec,
                   ref mut print_instruction_set,
                   ref mut personal,
                   ref mut overlay,
                   .. } = *gamedata;
    if let &mut Some(ref mut boardcodec) = boardcodec {
        let card_images = in_game::card_images(result_map);
        if let Some(ref mut _player) = boardcodec.players.get_mut(*page_index) {
            match gamedata.guistate {
                app::GuiState::Game(GameState::ShowDraft) => {
                    show_draft(ui, ids, print_instruction_set, &appdata);
                }
                app::GuiState::Game(GameState::Spell) => {
                    spell(ui,
                          ids,
                          _player,
                          &card_images,
                          &appdata,
                          personal,
                          overlay,
                          result_map,
                          _action_tx);
                }
                app::GuiState::Game(GameState::TurnToSubmit) => {
                    turn_to_submit(ui, ids, _player, &card_images, &appdata, result_map);
                }
                _ => {}
            }
        }
    }

    //  draw_hand(ui, ids, gamedata, appdata, result_map);
}
fn spell(ui: &mut conrod::UiCell,
         ids: &Ids,
         player: &mut Player,
         card_images: &[Option<image::Id>; 27],
         appdata: &AppData,
         personal: &mut Option<Personal>,
         overlay: &mut bool,
         result_map: &HashMap<ResourceEnum, SupportIdType>,
         _action_tx: mpsc::Sender<OwnedMessage>) {
    if let &mut Some(ref mut _personal) = personal {
        let mut handvec = _personal.hand
            .clone()
            .iter()
            .map(|ref x| {
                let (_image_id, _rect, _theme) =
                    in_game::get_card_widget_image_portrait(x.clone().clone(),
                                                            card_images,
                                                            appdata);
                (x.clone().clone(), _image_id, _rect)
            })
            .collect::<Vec<(usize, image::Id, conrod::Rect)>>();
        if let (Some(&SupportIdType::ImageId(spinner_image)),
                Some(&SupportIdType::ImageId(rust_image)),
                Some(&SupportIdType::ImageId(icon_image))) =
            (result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
             result_map.get(&ResourceEnum::Sprite(Sprite::RUST)),
             result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
            let exitid = DragDropList::new(&mut handvec,
                                           Box::new(move |(_v_index, v_blowup, v_rect)| {
                let spinner_sprite = graphics_match::spinner_sprite();
                sample_drag_image::Button::image(v_blowup)
                    .source_rectangle(v_rect)
                    .toggle_image(rust_image.clone())
                    .spinner_image(spinner_image.clone(), spinner_sprite)
                    .w_h(100.0, 300.0)
            }),
                                           50.0)
                    .padded_h_of(ids.footer, 10.0)
                    .padded_w_of(ids.footer, 150.0)
                    .top_left_of(ids.footer)
                    .exit_id(Some(Some(ids.bodydragdroplistview)))
                    .set(ids.footerdragdroplistview, ui);
            if let Some((v_index, _, _)) = exitid {
                _personal.arranged.push((v_index, false, None));
            }
            _personal.hand = handvec.iter().map(|&(x_index, _, _)| x_index).collect::<Vec<usize>>();
            let icon_rect = spriteable_rect(graphics_match::gameicon_sprite(), 1.0);
            for _ in widget::Button::image(icon_image)
                    .source_rectangle(Rect::from_corners(icon_rect.0, icon_rect.1))
                    .right_from(ids.footerdragdroplistview, 0.0)
                    .set(ids.footer_overlay_but, ui) {
                *overlay = true;
            }
        }
    }


}
fn show_draft(ui: &mut conrod::UiCell,
              ids: &Ids,
              print_instruction_set: &mut Vec<bool>,
              app: &AppData) {
    let g_vec = (*app)
        .texts
        .instructions1
        .iter()
        .enumerate()
        .filter(|&(index, _)| if index < 4 { true } else { false })
        .zip((*app).texts.instructions2.iter())
        .map(|((_index, ref label), &(ref rect_tuple, ref oval_option))| {
                 Instruction(label, rect_tuple, oval_option, ids.footer)
             })
        .collect::<Vec<Instruction>>();
    if let Some(_pi) = print_instruction_set.get_mut(0) {
        if *_pi {
            *_pi = InstructionSet::new(&g_vec, (*app).texts.next)
                .parent_id(ids.footer)
                .label_color(color::WHITE)
                .set(ids.instructionview, ui);
        }
    }

}
fn turn_to_submit(ui: &mut conrod::UiCell,
                  ids: &Ids,
                  player: &mut Player,
                  card_images: &[Option<image::Id>; 27],
                  appdata: &AppData,
                  result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let mut handvec = player.hand
        .iter()
        .map(|x| {
                 let (_image_id, _rect, _) =
                in_game::get_card_widget_image_portrait(x.clone(), card_images, appdata);
                 (x.clone(), _image_id, _rect)
             })
        .collect::<Vec<(usize, image::Id, conrod::Rect)>>();
    if let (Some(&SupportIdType::ImageId(spinner_image)),
            Some(&SupportIdType::ImageId(rust_image))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
         result_map.get(&ResourceEnum::Sprite(Sprite::RUST))) {
        let exitid = DragDropList::new(&mut handvec,
                                       Box::new(move |(_v_index, v_blowup, v_rect)| {
            let spinner_sprite = graphics_match::spinner_sprite();
            sample_drag_image::Button::image(v_blowup)
                .source_rectangle(v_rect)
                .toggle_image(rust_image.clone())
                .spinner_image(spinner_image.clone(), spinner_sprite)
                .w_h(100.0, 300.0)
        }),
                                       50.0)
                .padded_wh_of(ids.footer, 10.0)
                .top_left_of(ids.footer)
                .exit_id(Some(Some(ids.body)))
                .set(ids.handview, ui);
        if let Some((v_index, _, _)) = exitid {
            player.arranged.push((v_index, false, None));
        }
        player.hand = handvec.iter().map(|&(x_index, _, _)| x_index).collect::<Vec<usize>>();
    }
}


#[allow(dead_code)]
fn page_next(gamedata: &mut GameData) {
    if gamedata.page_index + 1 >= gamedata.player_size {
        gamedata.page_index = 0;
        for i in (0usize..gamedata.page_vec.len()).rev() {
            if let Some(&mut (ref mut _page, _)) = gamedata.page_vec.get_mut(i) {
                if i < gamedata.player_size {
                    _page.reverse_flip();
                }
            }
        }

    } else {
        if let Some(&mut (ref mut x, _)) = gamedata.page_vec.get_mut(gamedata.page_index) {
            x.flip();
        }
        gamedata.page_index += 1;

    }
}
#[allow(dead_code)]
fn page_previous(gamedata: &mut GameData) {
    if gamedata.page_index as f32 - 1.0 < 0.0 {
        gamedata.page_index = gamedata.player_size - 1;
        for i in 0..gamedata.page_vec.len() {
            if let Some(&mut (ref mut _page, _)) = gamedata.page_vec.get_mut(i) {
                if i < gamedata.player_size - 1 {
                    _page.flip();
                }
            }
        }
    } else {
        gamedata.page_index -= 1;
        if let Some(&mut (ref mut x, _)) = gamedata.page_vec.get_mut(gamedata.page_index) {
            x.reverse_flip();
        }

    }
}
