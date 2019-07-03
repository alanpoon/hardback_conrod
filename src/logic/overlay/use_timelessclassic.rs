use conrod_core::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable,
             Color, text, Rect};
use cardgame_widgets::custom_widget::tabview;
use cardgame_widgets::custom_widget::full_cycle_sprite::FullCycleSprite;
use cardgame_widgets::custom_widget::image_panels::{Panelable, ImagePanels, ImageRectType};
use custom_widget::buy_list_item;
use cardgame_widgets::sprite::SpriteInfo;
use backend::codec_lib::codec::*;
use backend::codec_lib;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use app::{GameData, Ids, OverlayStatus, BoardStruct};
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::{cards, local};
use graphics_match;
use logic::in_game;
use instruction::Instruction;
pub struct PanelInfo<'a> {
    text: Option<String>,
    display_pic: Option<ImageRectType>,
    values: &'a Vec<(bool, &'a str, Color, text::font::Id, Rect, Rect, usize)>,
    list_selected: &'a mut HashSet<usize, RandomState>,
    widget_closure: Box<Fn((bool, &'a str, Color, text::font::Id, Rect, Rect, usize))
                           -> buy_list_item::ItemWidget<'a>>,
}
impl<'b> Panelable for PanelInfo<'b> {
    type BorderableBorderedWidget = buy_list_item::ItemWidget<'b>;
    fn text(&self) -> Option<String> {
        self.text.clone()
    }
    fn display_pic(&self) -> Option<ImageRectType> {
        self.display_pic
    }
    fn len(&self) -> usize {
        self.values.len()
    }
    fn list_selected<'a>(&'a self) -> &'a HashSet<usize, RandomState> {
        &self.list_selected
    }

    fn list_selected_mut<'a>(&'a mut self) -> &'a mut HashSet<usize, RandomState> {
        self.list_selected
    }
    fn apply_closure(&self, i: usize) -> Self::BorderableBorderedWidget {
        (*self.widget_closure)(self.values
                                   .get(i)
                                   .unwrap()
                                   .clone())
    }
    fn card_index(&self, i: usize) -> usize {
        self.values
            .get(i)
            .unwrap()
            .6
            .clone()
    }
}
pub fn render(w_id: tabview::Item,
              ids: &Ids,
              cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              ui: &mut conrod_core::UiCell) {
    let GameData { ref mut boardcodec,
                   ref player_index,
                   ref mut personal,
                   ref mut overlay_timeless_selected,
                   ref mut overlay_receivedimage,
                   ref mut overlay_blowup,
                   .. } = *gamedata;
    widget::Text::new(&appdata.texts.use_timelessclassic)
        .color(color::WHITE)
        .font_size(30)
        .h(40.0)
        .w_of(w_id.parent_id)
        .top_left_of(w_id.parent_id)
        .set(ids.overlay_subject, ui);
    //normal_stuff don't need mut borrow
    let mut normal_stuff: Vec<(Option<String>,
                               Option<ImageRectType>,
                               Vec<(bool, &str, Color, text::font::Id, Rect, Rect, usize)>)> = vec![];
    // let card_images = in_game::card_images(result_map);
    if let (Some(&SupportIdType::ImageId(cloudy)),
            Some(&SupportIdType::ImageId(coin_info)),
            Some(&SupportIdType::ImageId(coin_info270)),
            Some(&SupportIdType::ImageId(arrows_image)),
            Some(&SupportIdType::ImageId(rust_logo))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270)),
         result_map.get(&ResourceEnum::Sprite(Sprite::ARROWS)),
         result_map.get(&ResourceEnum::Sprite(Sprite::RUST))) {
        let (_l, _t, _r, _b, _c) = graphics_match::all_arrows(arrows_image);
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
                let vec_cards =
                    _p.timeless_classic
                        .iter()
                        .map(|x| {
                            let mut r = Some(x.clone());
                            for (_ci, _inkbool, _, _timeless) in _personal.arranged.clone() {
                                if *x == _ci {
                                    r = None;
                                }
                            }
                            r
                        })
                        .filter(|x| x.is_some())
                        .map(|x| {
                            let (_timeless, _str, _color, _font, _rect, _top_left_rect) =
                                in_game::get_tile_image_withcost(x.unwrap().clone(),
                                                                 cardmeta,
                                                                 appdata,
                                                                 result_map);
                            (_timeless,
                             _str,
                             _color,
                             _font,
                             _rect,
                             _top_left_rect,
                             x.unwrap().clone())
                        })
                        .collect::<Vec<(bool, &str, Color, text::font::Id, Rect, Rect, usize)>>();

                normal_stuff.push((Some(_p.name), Some((rust_logo, None, 0)), vec_cards));
            }
            if let Some(_player) = _boardcodec.players.get_mut(_player_index.clone()) {
                let mut vec_p = normal_stuff.iter()
                    .zip(overlay_timeless_selected.iter_mut())
                    .map(|(normal, list_selected)| {
                        PanelInfo {
                            text: normal.0.clone(),
                            display_pic: normal.1.clone(),
                            values: &normal.2,
                            list_selected: list_selected,
                            widget_closure: Box::new(move |(_timeless,
                                                            _string,
                                                            _color,
                                                            _font,
                                                            _rect,
                                                            _top_left_rect,
                                                            _)| {

                                buy_list_item::ItemWidget::new(_timeless,
                                                               &_string,
                                                               _rect,
                                                               _top_left_rect,
                                                               "timeless")
                                        .cloudy_image(cloudy)
                                        .coin_info(coin_info)
                                        .coin_info270(coin_info270)
                                        .alphabet_font_id(_font)
                                        .color(_color)
                            }),
                        }
                    })
                    .collect::<Vec<PanelInfo>>();
                ImagePanels::new(&mut vec_p, overlay_blowup)
                    .middle_of(w_id.parent_id)
                    .padded_w_of(w_id.parent_id, 20.0)
                    .y_item_height(100.0)
                    .x_item_list([100.0, 100.0, 22.0, 5.0])
                    .corner_arrow(_c)
                    .label_color(color::LIGHT_YELLOW)
                    .set(ids.overlay_image_panels, ui);
                match overlay_receivedimage[2] {
                    OverlayStatus::None => {
                        for _c in widget::Button::new()
                                .label(&appdata.texts.use_timelessclassic)
                                .h(40.0)
                                .mid_bottom_with_margin_on(w_id.parent_id, 20.0)
                                .set(ids.overlay_okbut, ui) {
                            overlay_receivedimage[2] = OverlayStatus::Loading;
                            for &PanelInfo { ref values, ref list_selected, .. } in vec_p.iter() {
                                //list_image ->Vec<ImageRectType>
                                for _selected in list_selected.iter() {
                                    if let Some(&(_timeless,
                                                  _,
                                                  _color,
                                                  _font,
                                                  _rect,
                                                  _top_left_rect,
                                                  _ci)) = values.get(_selected.clone()) {
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
