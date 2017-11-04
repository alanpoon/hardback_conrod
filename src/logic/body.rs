use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use conrod::widget::list_select::Event;
use conrod::widget::list::Right;
use conrod::widget::list::Fixed;
use conrod::widget::primitive::image::Image;
use cardgame_widgets::custom_widget::animated_button;
use cardgame_widgets::custom_widget::dragdrop_list::DragDropList;
use cardgame_widgets::custom_widget::sample_drag_image_primitive;
use backend::codec_lib::codec::*;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Font, Sprite};
use backend::metat::cards;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids};

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
                app::GuiState::Game(GameState::ShowDraft) => {
                    show_draft(ui, ids, _player, &card_images, result_map);
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
              player: &mut Player,
              card_images: &[Option<image::Id>; 27],
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let item_h = 300.0;
    let (mut items, scrollbar) = widget::List::flow_down(player.draft.len())
        .item_size(item_h)
        .middle_of(ids.body)
        .wh_of(ids.body)
        .scrollbar_next_to()
        .set(ids.listview, ui);
    let mut draft_iter = player.draft.iter();
    while let (Some(item), Some(card_index)) = (items.next(ui), draft_iter.next()) {}

}
fn turn_to_submit(ui: &mut conrod::UiCell,
                  ids: &Ids,
                  player: &mut Player,
                  card_images: &[Option<image::Id>; 27],
                  appdata: &AppData,
                  result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let mut handvec = player.arranged
        .iter()
        .map(|&(ref x, _)| {
                 let image_primitive =
                in_game::get_card_widget_image_portrait(x.clone(), card_images, appdata);
                 (x.clone(), image_primitive)
             })
        .collect::<Vec<(usize, image::Id)>>();
    if let (Some(&SupportIdType::ImageId(spinner_image)),
            Some(&SupportIdType::ImageId(rust_image))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
         result_map.get(&ResourceEnum::Sprite(Sprite::RUST))) {
        let exitid =
            DragDropList::new(&mut handvec,
                              Box::new(move |(v_index, v_blowup)| {
                                           sample_drag_image_primitive::Button::image(v_blowup)
                                               .toggle_image(rust_image.clone())
                                               .spinner_image(spinner_image.clone())
                                               .w_h(100.0, 300.0)
                                       }),
                              50.0)
                    .padded_wh_of(ids.body, 10.0)
                    .top_left_of(ids.body)
                    .exit_id(Some(Some(ids.footer)))
                    .set(ids.arrangedview, ui);
        if let Some((v_index, _)) = exitid {
            player.hand.push(v_index);
        }
        player.hand = handvec.iter().map(|&(x_index, _)| x_index).collect::<Vec<usize>>();
    }
}
