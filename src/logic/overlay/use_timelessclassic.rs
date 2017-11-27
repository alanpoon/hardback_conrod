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
pub struct PanelInfo<'a> {
    text: Option<String>,
    display_pic: Option<ImageRectType>,
    list_image: Vec<ImageRectType>,
    list_selected: &'a mut HashSet<usize, RandomState>,
}
impl<'b> Panelable for PanelInfo<'b> {
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
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              _action_tx: mpsc::Sender<OwnedMessage>,
              ui: &mut conrod::UiCell) {
    let GameData { ref mut boardcodec,
                   ref player_index,
                   ref mut personal,
                   ref mut overlay_timeless_selected,
                   ref mut overlay_receivedimage,
                   .. } = *gamedata;

    //normal_stuff don't need mut borrow
    let mut normal_stuff: Vec<(Option<String>, Option<ImageRectType>, Vec<ImageRectType>)> = vec![];
    let card_images = in_game::card_images(result_map);
    if let Some(&SupportIdType::ImageId(rust_logo)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::RUST)) {
        if let (&mut Some(ref mut _boardcodec),
                &Some(ref _player_index),
                &mut Some(ref mut _personal)) = (boardcodec, player_index, personal) {
            //remove all timelessclassic from personal's arranged
            _personal.arranged = _personal.arranged
                .iter()
                .filter(|&&(ref _c, _, ref _optstr, ref _t)| !_t.clone())
                .map(|x| x.clone())
                .collect::<Vec<(usize, bool, Option<String>, bool)>>();
            for _p in _boardcodec.players.clone() {
                let vec_cards = _p.timeless_classic
                    .iter()
                    .map(|x| {
                        let mut r = None;
                        for (_ci, _inkbool, _, _timeless) in _personal.arranged.clone() {
                            if *x != _ci {
                                let (_image_id, _rect, _) =
                                    in_game::get_card_widget_image_flexible(x.clone(),
                                                                            &card_images,
                                                                            appdata);
                                let top_left = _rect.top_left();
                                let btm_right = _rect.bottom_right();
                                r = Some((_image_id, Some((top_left, btm_right)), _ci));
                            }
                        }
                        r
                    })
                    .filter(|x| if let &Some(_) = x { true } else { false })
                    .map(|x| x.unwrap())
                    .collect::<Vec<ImageRectType>>();

                normal_stuff.push((Some(_p.name), Some((rust_logo, None, 0)), vec_cards));
            }
            if let Some(_player) = _boardcodec.players.get_mut(_player_index.clone()) {
                let mut vec_p = normal_stuff.iter()
                    .zip(overlay_timeless_selected.iter_mut())
                    .map(|(normal, list_selected)| {
                        PanelInfo {
                            text: normal.0.clone(),
                            display_pic: normal.1.clone(),
                            list_image: normal.2.clone(),
                            list_selected: list_selected,
                        }
                    })
                    .collect::<Vec<PanelInfo>>();
                ImagePanels::new(&mut vec_p)
                    .middle_of(w_id.parent_id)
                    .padded_w_of(w_id.parent_id, 20.0)
                    .y_item_height(100.0)
                    .x_item_list([100.0, 100.0, 22.0, 5.0])
                    .set(ids.overlay_image_panels, ui);
                match overlay_receivedimage[2] {
                    OverlayStatus::None => {
                        for _c in widget::Button::new()
                                .label(&appdata.texts.use_timelessclassic)
                                .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                                .set(ids.overlay_okbut, ui) {
                            overlay_receivedimage[2] = OverlayStatus::Loading;
                            for &PanelInfo { ref list_image, ref list_selected, .. } in
                                vec_p.iter() {
                                //list_image ->Vec<ImageRectType>
                                for _selected in list_selected.iter() {
                                    if let Some(&(_, _, ref _ci)) =
                                        list_image.get(_selected.clone()) {
                                        _personal.arranged.push((_ci.clone(), false, None, true));
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }

            }
        }
    }

}
