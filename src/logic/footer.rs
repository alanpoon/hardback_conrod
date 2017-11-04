use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use conrod::widget::list_select::Event;
use conrod::widget::list::Right;
use conrod::widget::list::Fixed;
use hardback_meta::app::{AppData, ResourceEnum, Font, Sprite};
use hardback_meta::cards;
use cardgame_widgets::custom_widget::animated_button;
use cardgame_widgets::custom_widget::dragdrop_list::DragDropList;
use cardgame_widgets::custom_widget::sample_drag_image;
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids};
use backend::OwnedMessage;
use backend::SupportIdType;
use graphics_match::button;
use logic::in_game;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {
    let GameData { ref page_index, ref mut boardcodec, .. } = *gamedata;
    if let &mut Some(ref mut boardcodec) = boardcodec {
        let card_images = in_game::card_images(result_map);
        if let Some(ref mut _player) = boardcodec.players.get_mut(*page_index) {
            match gamedata.guistate {
                app::GuiState::Game(GameState::TurnToSubmit) => {
                    turn_to_submit(ui, ids, _player, &card_images, result_map);
                }
                _ => {}
            }
        }
    }

    //  draw_hand(ui, ids, gamedata, appdata, result_map);
}
fn turn_to_submit(ui: &mut conrod::UiCell,
                  ids: &Ids,
                  player: &mut Player,
                  card_images: &[Option<image::Id>; 27],
                  result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let mut handvec = player.hand
        .iter()
        .map(|x| (x.clone(), card_images[x.clone()].clone().unwrap()))
        .collect::<Vec<(usize, image::Id)>>();
    if let (Some(&SupportIdType::ImageId(spinner_image)),
            Some(&SupportIdType::ImageId(rust_image))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
         result_map.get(&ResourceEnum::Sprite(Sprite::RUST))) {
        let exitid = DragDropList::new(&mut handvec,
                                       Box::new(move |(v_index, v_blowup)| {
                                                    sample_drag_image::Button::image(v_blowup)
                                                        .toggle_image(rust_image.clone())
                                                        .spinner_image(spinner_image.clone())
                                                        .w_h(100.0, 300.0)
                                                }),
                                       50.0)
                .padded_wh_of(ids.footer, 10.0)
                .top_left_of(ids.footer)
                .exit_id(Some(Some(ids.body)))
                .set(ids.handview, ui);
        if let Some((v_index, _)) = exitid {
            player.arranged.push((v_index, None));
        }
        player.hand = handvec.iter().map(|&(x_index, _)| x_index).collect::<Vec<usize>>();
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
