use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use conrod::position::rect::Rect;
use conrod::widget::primitive::image::Image;
use conrod::widget::envelope_editor::EnvelopePoint;
use cardgame_widgets::custom_widget::dragdrop_list::DragDropList;
use cardgame_widgets::custom_widget::sample_drag_image;
use cardgame_widgets::custom_widget::image_hover::{Hoverable, ImageHover};
use backend::codec_lib::codec::*;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use std::collections::HashMap;
use futures::sync::mpsc;
use app::{self, GameData, Ids};
use logic::in_game;

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
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              _action_tx: mpsc::Sender<OwnedMessage>) {
    let GameData { ref page_index, ref mut boardcodec, .. } = *gamedata;
    if let &mut Some(ref mut boardcodec) = boardcodec {
        let card_images = in_game::card_images(result_map);
        if let Some(ref mut _player) = boardcodec.players.get_mut(*page_index) {
            match gamedata.guistate {
                app::GuiState::Game(GameState::ShowDraft) => {
                    show_draft(ui, ids, _player, &card_images, &appdata, result_map);
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
              appdata: &AppData,
              _result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let item_h = 180.0;
    println!("draft {:?}",player.draft.clone());
    let (mut items, scrollbar) = widget::List::flow_right(player.draft.len())
        .item_size(item_h)
        .middle_of(ids.body)
        .h(220.0)
        .padded_w_of(ids.body, 20.0)
        .scrollbar_next_to()
        .set(ids.listview, ui);
    if let Some(s) = scrollbar {
        s.set(ui)
    }
    let mut draft_iter = player.draft.iter();
    while let (Some(item), Some(card_index)) = (items.next(ui), draft_iter.next()) {
        let (_image_id, _rect) =
            in_game::get_card_widget_image_portrait(card_index.clone(), card_images, appdata);
        //zoom rect
        let mut top_left_c = _rect.top_left().clone();
        top_left_c.set_x(_rect.top_left().get_x() + 100.0);
        top_left_c.set_y(_rect.top_left().get_y() + 80.0);
        let btm_right = _rect.bottom_right().clone();
        let _zoom_rect = Rect::from_corners(top_left_c, btm_right);
        let _ih = ImageHoverable(Image::new(_image_id).source_rectangle(_rect),
                                 Some(Image::new(_image_id).source_rectangle(_zoom_rect)),
                                 None);
        let j = ImageHover::new(_ih);
        item.set(j, ui);
    }

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
                .padded_wh_of(ids.body, 10.0)
                .top_left_of(ids.body)
                .exit_id(Some(Some(ids.footer)))
                .set(ids.arrangedview, ui);
        if let Some((v_index, _, _)) = exitid {
            player.hand.push(v_index);
        }
        player.hand = handvec.iter().map(|&(x_index, _, _)| x_index).collect::<Vec<usize>>();
    }
}
