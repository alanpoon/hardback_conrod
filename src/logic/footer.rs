use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use cardgame_widgets::custom_widget::dragdrop_list::DragDropList;
use cardgame_widgets::custom_widget::sample_drag_image;
use cardgame_widgets::custom_widget::instructionset::InstructionSet;
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use app::{self, GameData, Ids};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::cards;
use backend::meta::local;
use logic::in_game;
use instruction::Instruction;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              _action_tx: mpsc::Sender<OwnedMessage>) {
    let GameData { ref page_index, ref mut boardcodec, ref mut print_instruction_set, .. } =
        *gamedata;

    if let &mut Some(ref mut boardcodec) = boardcodec {
        let card_images = in_game::card_images(result_map);
        if let Some(ref mut _player) = boardcodec.players.get_mut(*page_index) {
            match gamedata.guistate {
                app::GuiState::Game(GameState::ShowDraft) => {
                    show_draft(ui, ids, print_instruction_set, &appdata);
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
        println!("print_instruction0");
        if *_pi {
            println!("print_instruction");
            *_pi = InstructionSet::new(&g_vec, (*app).texts.next)
                .parent_id(ids.footer)
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
                 let (_image_id, _rect) =
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
            sample_drag_image::Button::image(v_blowup)
                .source_rectangle(v_rect)
                .toggle_image(rust_image.clone())
                .spinner_image(spinner_image.clone())
                .w_h(100.0, 300.0)
        }),
                                       50.0)
                .padded_wh_of(ids.footer, 10.0)
                .top_left_of(ids.footer)
                .exit_id(Some(Some(ids.body)))
                .set(ids.handview, ui);
        if let Some((v_index, _, _)) = exitid {
            player.arranged.push((v_index, None));
        }
        player.hand = handvec.iter().map(|&(x_index, _, _)| x_index).collect::<Vec<usize>>();
    }
}
/*           
fn draw_hand(ui: &mut conrod::UiCell,
             ids: &Ids,
             mut gamedata: &mut GameData,
             appdata: &AppData,
             result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let wh = ui.wh_of(ids.footer).unwrap();
    let z = button::get_style();
    if let Some(&SupportIdType::ImageId(but_logo)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::BUTTON)) {
        if animated_button::AnimatedButton::image(but_logo)
               .label(appdata.texts.previous)
               .normal_rect(z.src_rect(19.0))
               .hover_rect(z.src_rect(20.0))
               .press_rect(z.src_rect(20.0))
               .top_left_of(ids.footer)
               .w(0.2 * wh[0])
               .h(0.3 * wh[1])
               .set(ids.footerprevious, ui)
               .was_clicked() {
            println!("aaaa");
            page_previous(gamedata);
        }
        if animated_button::AnimatedButton::image(but_logo)
               .label(appdata.texts.next)
               .normal_rect(z.src_rect(19.0))
               .hover_rect(z.src_rect(20.0))
               .press_rect(z.src_rect(20.0))
               .top_right_of(ids.footer)
               .w(0.2 * wh[0])
               .h(0.3 * wh[1])
               .set(ids.footernext, ui)
               .was_clicked() {
            page_next(gamedata);
        };
    }
}
*/
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
