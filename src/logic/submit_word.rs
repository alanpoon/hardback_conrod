use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use hardback_meta::app::{AppData, ResourceEnum, Font, Sprite};
use hardback_meta::cards;
use cardgame_widgets::custom_widget::dragdrop_list::DragDropList;
use cardgame_widgets::custom_widget::sample_drag_image;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::codec_lib::codec::*;
use graphics_match::button;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {
    match gamedata.gamestate {
        app::GameState::SubmitWord => {
            draw_cards(ui, ids, gamedata, appdata, result_map);
        }
        _ => {}
    }
}
fn draw_cards(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    widget::Canvas::new()
        .color(color::TRANSPARENT)
        .flow_down(&[(ids.body, widget::Canvas::new().color(color::TRANSPARENT)),
                     (ids.footer, widget::Canvas::new().color(color::DARK_GREEN).length(100.0))])
        .set(ids.master, ui);
    let card_images = card_images(result_map);
    let GameData { ref page_index, ref mut boardcodec, .. } = *gamedata;
    if let &mut Some(ref mut boardcodec) = boardcodec {
        if let Some(ref mut _player) = boardcodec.players.get_mut(*page_index) {
            draw_hand(ui, ids, _player, &card_images, result_map);
            draw_arranged(ui, ids, _player, &card_images, result_map);
        }
    }
}
fn draw_hand(ui: &mut conrod::UiCell,
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
                .exit_id(Some(Some(ids.arranged_view)))
                .set(ids.hand_view, ui);
        if let Some((v_index, _)) = exitid {
            player.arranged.push((v_index,None));
        }
        player.hand = handvec.iter().map(|&(x_index, _)| x_index).collect::<Vec<usize>>();
    }

}
fn draw_arranged(ui: &mut conrod::UiCell,
                 ids: &Ids,
                 player: &mut Player,
                 card_images: &[Option<image::Id>; 27],
                 result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let mut handvec = player.arranged
        .iter()
        .map(|&(ref x,_)| (x.clone(), card_images[x.clone()].clone().unwrap()))
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
                .padded_wh_of(ids.body, 10.0)
                .top_left_of(ids.body)
                .exit_id(Some(Some(ids.hand_view)))
                .set(ids.hand_view, ui);
        if let Some((v_index, _)) = exitid {
            player.hand.push(v_index);
        }
        player.hand = handvec.iter().map(|&(x_index, _)| x_index).collect::<Vec<usize>>();
    }

}
fn card_images(result_map: &HashMap<ResourceEnum, SupportIdType>)
               -> [Option<conrod::image::Id>; 27] {
    let mut j = [None; 27];
    if let (Some(&SupportIdType::ImageId(cards1)),
            Some(&SupportIdType::ImageId(cards2)),
            Some(&SupportIdType::ImageId(cards3)),
            Some(&SupportIdType::ImageId(cards4)),
            Some(&SupportIdType::ImageId(cards5)),
            Some(&SupportIdType::ImageId(cards6)),
            Some(&SupportIdType::ImageId(cards7)),
            Some(&SupportIdType::ImageId(cards8)),
            Some(&SupportIdType::ImageId(cards9)),
            Some(&SupportIdType::ImageId(cards10)),
            Some(&SupportIdType::ImageId(cards11)),
            Some(&SupportIdType::ImageId(cards12)),
            Some(&SupportIdType::ImageId(cards13)),
            Some(&SupportIdType::ImageId(cards14)),
            Some(&SupportIdType::ImageId(cards15)),
            Some(&SupportIdType::ImageId(cards16)),
            Some(&SupportIdType::ImageId(cards17)),
            Some(&SupportIdType::ImageId(cards18)),
            Some(&SupportIdType::ImageId(cards19)),
            Some(&SupportIdType::ImageId(cards20)),
            Some(&SupportIdType::ImageId(cards21)),
            Some(&SupportIdType::ImageId(cards22)),
            Some(&SupportIdType::ImageId(cards23)),
            Some(&SupportIdType::ImageId(cards24)),
            Some(&SupportIdType::ImageId(cards25)),
            Some(&SupportIdType::ImageId(cards26)),
            Some(&SupportIdType::ImageId(cards27))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::CARDS1)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS2)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS3)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS4)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS5)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS6)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS7)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS8)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS9)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS10)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS11)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS12)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS13)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS14)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS15)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS16)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS17)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS18)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS19)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS20)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS21)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS22)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS23)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS24)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS25)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS26)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS27))) {
        j[0] = Some(cards1);
        j[1] = Some(cards2);
        j[2] = Some(cards3);
        j[3] = Some(cards4);
        j[4] = Some(cards5);
        j[5] = Some(cards6);
        j[6] = Some(cards7);
        j[7] = Some(cards8);
        j[8] = Some(cards9);
        j[9] = Some(cards10);
        j[10] = Some(cards11);
        j[11] = Some(cards12);
        j[12] = Some(cards13);
        j[13] = Some(cards14);
        j[14] = Some(cards15);
        j[15] = Some(cards16);
        j[16] = Some(cards17);
        j[17] = Some(cards18);
        j[18] = Some(cards19);
        j[19] = Some(cards20);
        j[20] = Some(cards21);
        j[21] = Some(cards22);
        j[22] = Some(cards23);
        j[23] = Some(cards24);
        j[24] = Some(cards25);
        j[25] = Some(cards26);
        j[26] = Some(cards27);
    }
    j
}
