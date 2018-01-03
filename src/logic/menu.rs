use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, Labelable};
use std::collections::HashMap;
use futures::sync::mpsc;
use app::{BoardStruct, GameData, Ids, GuiState};
use cardgame_widgets::custom_widget::animated_canvas;
use custom_widget::show_draft_item;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use logic::in_game;
use backend::codec_lib;
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
            Some(&SupportIdType::ImageId(coin_info270))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270))) {
        let w = ui.w_of(ids.master).unwrap();
        let (mut items, _scrollbar) = widget::List::flow_right(4)
            .item_size(w / 12.0)
            .h(appdata.convert_h(200.0))
            .mid_top_with_margin_on(ids.master, appdata.convert_h(40.0))
            .set(ids.menu_title_list1, ui);
        let word_arr = vec![22,48,86,143];
        let mut word_iter = word_arr.iter();
        while let (Some(item), Some(&card_index)) = (items.next(ui), word_iter.next()) {
            let (_timeless, _str, _color, _font, _rect) =
                in_game::get_tile_image_withcost(card_index, cardmeta, appdata, result_map);
            let j = show_draft_item::ItemWidget::new(_timeless, _str, _rect, "timeless")
                .cloudy_image(cloudy)
                .coin_info(coin_info)
                .coin_info270(coin_info270)
                .alphabet_font_id(_font)
                .color(_color);
            item.set(j, ui);
        }
        let blackjack_arr = vec![127,32,140,2,99,43,0,142,118];
        let mut blackjack_iter = blackjack_arr.iter();
        let (mut items2, _scrollbar) = widget::List::flow_right(blackjack_arr.len())
            .item_size(w / 12.0)
            .h(appdata.convert_h(200.0))
            .down_from(ids.menu_title_list1, appdata.convert_h(20.0))
            .set(ids.menu_title_list2, ui);
        let mut c =0;
        while let (Some(item), Some(&card_index)) = (items2.next(ui), blackjack_iter.next()) {
            println!("c :{:?}",c);
            c+=1;
            let (_timeless, _str, _color, _font, _rect) =
                in_game::get_tile_image_withcost(card_index, cardmeta, appdata, result_map);
            let j = show_draft_item::ItemWidget::new(_timeless, _str, _rect, "timeless")
                .cloudy_image(cloudy)
                .coin_info(coin_info)
                .coin_info270(coin_info270)
                .alphabet_font_id(_font)
                .color(_color);
            item.set(j, ui);
        }
    }
    if widget::Button::new()
           .wh(appdata.convert_dim([400.0, 100.0]))
           .bottom_left_with_margin_on(ids.master, appdata.convert_h(20.0))
           .rgb(0.4, 0.75, 0.6)
           .label("Multiplayer")
           .set(ids.menubut_multiplayer, ui)
           .was_clicked() {
        gamedata.guistate = GuiState::Lobby;
    }

}
