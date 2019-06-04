use conrod_core::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable,
             Borderable, Rect};
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::sprite::{SpriteInfo, spriteable_rect};
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids, GuiState};
use graphics_match;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use logic;
pub fn render(ui: &mut conrod_core::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    if gamedata.overlay_human {
        if let Some(&SupportIdType::ImageId(keypad_image)) =
            result_map.get(&ResourceEnum::Sprite(Sprite::KEYPAD)) {
            let default_color = color::LIGHT_BLUE;
            let close_rect = spriteable_rect(graphics_match::keypad_sprite(), 2.0);
            if animated_canvas::Canvas::new()
                   .bottom_right_with_margins_on(ids.master, appdata.convert_h(210.0), 30.0)
                   .wh([120.0, 200.0])
                   .color(color::LIGHT_BLUE)
                   .parent(ids.master)
                   .close_icon_color(color::YELLOW)
                   .close_icon_dim([30.0, 30.0])
                   .close_icon(keypad_image)
                   .close_icon_src_rect(Rect::from_corners(close_rect.0, close_rect.1))
                   .frame_rate(30)
                   .set(ids.overlay_human, ui)
                   .is_done() {
                gamedata.overlay_human = false;
            }
            let mut players_name = vec![];
            if let Some(ref _boardcodec) = gamedata.boardcodec {
                players_name = _boardcodec.players
                    .iter()
                    .map(|ref s| s.name.clone())
                    .collect::<Vec<String>>();
            }

            let item_h = 40.0;
            let font_size = item_h as conrod_core::FontSize / 2;
            let (mut events, _scrollbar) = widget::ListSelect::single(gamedata.player_size)
                .flow_down()
                .item_size(item_h)
                .scrollbar_next_to()
                .wh_of(ids.overlay_human)
                .middle_of(ids.overlay_human)
                .set(ids.overlay_human_list, ui);
            while let Some(event) = events.next(ui, |i| if gamedata.page_index == i {
                true
            } else {
                false
            }) {
                use conrod_core::widget::list_select::Event;
                match event {
                    // For the `Item` events we instantiate the `List`'s items.
                    Event::Item(item) => {
                        let (color, label_color) = (conrod_core::color::LIGHT_GREY,
                                                    conrod_core::color::BLACK);
                        let button = widget::Button::new()
                            .border(0.0)
                            .color(color)
                            .label(players_name.get(item.i).unwrap())
                            .label_font_size(font_size)
                            .label_color(label_color);
                        item.set(button, ui);
                    }
                    Event::Selection(selected_id) => {
                        if let Some(_go_to_page) = gamedata.go_to_page_index.clone() {
                            if gamedata.page_index.clone() < _go_to_page {
                                page_next(gamedata, _go_to_page);
                            } else if gamedata.page_index.clone() > _go_to_page {
                                page_previous(gamedata, _go_to_page);
                            }
                        }
                    }
                    _ => {}
                }
            }

        }
    }

}

#[allow(dead_code)]
fn page_next(gamedata: &mut GameData, go_to_page: usize) {
    if gamedata.page_index + 1 >= gamedata.player_size {
        gamedata.page_index = 0;
        for i in (0usize..gamedata.page_vec.len()).rev() {
            if let Some(&mut (ref mut _page, _)) = gamedata.page_vec.get_mut(i) {
                if i < gamedata.player_size {
                    _page.reverse_flip();
                    gamedata.page_index -= 1;
                }
            }
        }
    } else {
        if let Some(&mut (ref mut x, _)) = gamedata.page_vec.get_mut(gamedata.page_index) {
            for _ in gamedata.page_index..go_to_page {
                x.flip();
                gamedata.page_index += 1;
            }
        }
    }
}
#[allow(dead_code)]
fn page_previous(gamedata: &mut GameData, go_to_page: usize) {
    if gamedata.page_index as f32 - 1.0 < 0.0 {
        gamedata.page_index = gamedata.player_size - 1;
        for i in 0..gamedata.page_vec.len() {
            if let Some(&mut (ref mut _page, _)) = gamedata.page_vec.get_mut(i) {
                if i < gamedata.player_size - 1 {
                    _page.flip();
                    gamedata.page_index += 1;
                }
            }
        }
    } else {
        gamedata.page_index -= 1;
        if let Some(&mut (ref mut x, _)) = gamedata.page_vec.get_mut(gamedata.page_index) {
            for _ in gamedata.page_index.clone()..go_to_page {
                x.reverse_flip();
                gamedata.page_index -= 1;
            }
        }
    }
}
