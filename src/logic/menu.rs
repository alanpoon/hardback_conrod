use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, Labelable};
use std::collections::HashMap;
use futures::sync::mpsc;
use app::{BoardStruct, GameData, Ids, GuiState};
use cardgame_widgets::custom_widget::animated_canvas;
use custom_widget::show_draft_item;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use graphics_match;
use logic::in_game;
use logic;
use backend::codec_lib;
use backend::codec_lib::codec::{ConnectionStatus, ConnectionError};
#[allow(unused_mut)]
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    animated_canvas::Canvas::new().color(color::LIGHT_ORANGE).frame_rate(30).set(ids.master, ui);
    if let (Some(&SupportIdType::ImageId(cloudy)),
            Some(&SupportIdType::ImageId(coin_info)),
            Some(&SupportIdType::ImageId(coin_info270)),
            Some(&SupportIdType::ImageId(unofficial)),
            Some(&SupportIdType::ImageId(icon_image))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270)),
         result_map.get(&ResourceEnum::Sprite(Sprite::UNOFFICIAL)),
         result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
        let w = ui.w_of(ids.master).unwrap();
        let word_arr = vec![41, 140, 17, 72,1, 108, 126,44];
        let (mut items, _scrollbar) = widget::List::flow_right(word_arr.len())
            .item_size(w / 10.0)
            .w(w)
            .h(appdata.convert_h(200.0))
            .mid_top_with_margin_on(ids.master, appdata.convert_h(40.0))
            .set(ids.menu_title_list1, ui);

        let mut word_iter = word_arr.iter();
        while let (Some(item), Some(&card_index)) = (items.next(ui), word_iter.next()) {
            let (_timeless, _str, _color, _font, _rect, _top_lefticon) =
                in_game::get_tile_image_withcost(card_index, cardmeta, appdata, result_map);
            let j =
                show_draft_item::ItemWidget::new(_timeless, _str, _rect, _top_lefticon, "timeless")
                    .cloudy_image(cloudy)
                    .coin_info(coin_info)
                    .coin_info270(coin_info270)
                    .alphabet_font_id(_font)
                    .color(_color);
            item.set(j, ui);
        }
        widget::Image::new(unofficial)
            .wh(appdata.convert_dim([280.0, 100.0]))
            .down_from(ids.menu_title_list1, 20.0)
            .set(ids.unofficial_logo, ui);


        match &gamedata.connection_status {
            &ConnectionStatus::Ok => {
                if widget::Button::new()
                       .wh(appdata.convert_dim([400.0, 100.0]))
                       .bottom_left_with_margins_on(ids.master, 100.0, 20.0)
                       .rgb(0.4, 0.75, 0.6)
                       .label("Multiplayer")
                       .set(ids.menubut_multiplayer, ui)
                       .was_clicked() {
                    gamedata.guistate = GuiState::Lobby;
                }
            }
            _ => {
                widget::Text::new(appdata.texts.waiting_for_connection)
                    .font_size(40)
                    .bottom_left_with_margins_on(ids.master, 100.0, 20.0)
                    .color(color::LIGHT_GREEN)
                    .set(ids.menu_waiting_connection, ui);
                logic::notification::render(ui, ids, ids.master, gamedata.notification.clone());
            }
        }
        for _ in widget::Button::image(icon_image)
                .source_rectangle(graphics_match::gameicons_rect(10.0))
                .w_h(80.0, 80.0)
                .bottom_left_with_margin_on(ids.master, 20.0)
                .set(ids.footer_overlay_but3, ui) {
            gamedata.overlay_exit = true;
        }
        widget::Text::new(gamedata.version)
            .bottom_right_with_margin_on(ids.master, 50.0)
            .color(color::WHITE)
            .font_size(40)
            .set(ids.menu_version_num, ui);
    }
}
