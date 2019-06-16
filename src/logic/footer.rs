use conrod_core::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable,
             Borderable, Rect, text, Color};
//
use conrod_core::widget::primitive::image::Image;
use cardgame_widgets::custom_widget::image_hover::{Hoverable, ImageHover};
use cardgame_widgets::custom_widget::bordered_image::Bordered;
use cardgame_widgets::custom_widget::arrange_list::{ArrangeList, ExitBy};
use custom_widget::arrange_list_tile::{ArrangeTuple,ItemWidget};
use custom_widget::buy_list_item;
use cardgame_widgets::custom_widget::instructionset::InstructionSet;
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::text::get_font_size_hn;
use cardgame_widgets::custom_widget::player_info::list::List;
use cardgame_widgets::custom_widget::player_info::item::IconStruct;
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use std::time::Instant;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::{self, local};
use graphics_match;
use graphics_match::ImageHoverable;
use logic::in_game;
use instruction::Instruction;
use cardgame_widgets::custom_widget::promptview::PromptSendable;
use app::{BoardStruct, PromptSender};
use backend::codec_lib;
pub fn render(ui: &mut conrod_core::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let GameData { ref page_index,
                   ref mut boardcodec,
                   ref mut print_instruction_set,
                   ref mut print_instruction_cache,
                   ref mut personal,
                   ref mut overlay_blowup,
                   ref mut overlay,
                   ref mut overlay2,
                   ref mut overlay_chat,
                   ref mut overlay_exit,
                   ref mut overlay_human,
                   ref mut buy_selected,
                   ref mut last_send,
                   ref player_index,
                   ref mut spell_which_arrangelist,
                   .. } = *gamedata;
    if let &mut Some(ref mut boardcodec) = boardcodec {
        if let Some(ref mut _player) = boardcodec.players.get_mut(*page_index) {
            match gamedata.guistate {
                app::GuiState::Game(GameState::ShowDraft) => {
                    show_draft(ui,
                               ids,
                               print_instruction_set,
                               print_instruction_cache,
                               &appdata);
                }
                app::GuiState::Game(GameState::Spell) => {
                    
                    if *page_index != player_index.unwrap() {
                        view_others(ui,
                                    ids,
                                    _player.clone(),
                                    &appdata,
                                    &cardmeta,
                                    spell_which_arrangelist,
                                    overlay_blowup,
                                    buy_selected,
                                    overlay,
                                    overlay_chat,
                                    overlay_exit,
                                    overlay_human,
                                    result_map);
                    } else {
                        spell(ui,
                              ids,
                              personal,
                              &appdata,
                              &cardmeta,
                              spell_which_arrangelist,
                              overlay_blowup,
                              overlay,
                              overlay_chat,
                              overlay_exit,
                              overlay_human,
                              last_send,
                              result_map);
                    }
                }
                app::GuiState::Game(GameState::TurnToSubmit) => {
                    spell(ui,
                          ids,
                          personal,
                          &appdata,
                          &cardmeta,
                          spell_which_arrangelist,
                          overlay_blowup,
                          overlay,
                          overlay_chat,
                          overlay_exit,
                          overlay_human,
                          last_send,
                          result_map);
                }
                app::GuiState::Game(GameState::Buy) => {
                    buy(ui,
                        ids,
                        _player,
                        overlay2,
                        buy_selected,
                        appdata,
                        result_map);
                }
                app::GuiState::Game(GameState::TrashOther(_)) => {
                    trash_other(ui,
                                ids,
                                _player,
                                overlay2,
                                buy_selected,
                                appdata,
                                result_map);
                }
                app::GuiState::Game(GameState::WaitForReply) => {
                    buy(ui,
                        ids,
                        _player,
                        overlay2,
                        buy_selected,
                        appdata,
                        result_map);
                }
                _ => {}
            }
        }
    }

    //  draw_hand(ui, ids, gamedata, appdata, result_map);
}
fn view_others(ui: &mut conrod_core::UiCell,
               ids: &Ids,
               player: Player,
               appdata: &AppData,
               cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
               spell_which_arrangelist: &mut Option<widget::Id>,
               overlay_blowup: &mut Option<usize>,
               buyselected: &mut Option<usize>,
               overlay: &mut bool,
               overlay_chat: &mut bool,
               overlay_exit: &mut bool,
               overlay_human: &mut bool,
               result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let handvec = player.hand
        .clone()
        .iter()
        .map(|ref x| {
                 let (_timeless, _string, _color, _app_font, _rect, _top_left_rect) =
                in_game::get_tile_image_withcost(*x.clone(), cardmeta, appdata, result_map);
                 (*x.clone(), _timeless, _string.to_owned(),None, _color, _app_font, _rect, _top_left_rect,false)
             })
        .collect::<Vec<ArrangeTuple>>();
    let footer_list_w = ui.w_of(ids.footer).unwrap() - 300.0;
    let item_h = footer_list_w / 7.0;
    let (mut events, scrollbar) = widget::ListSelect::single(handvec.len())
        .flow_right()
        .item_size(item_h)
        .padded_h_of(ids.footer, 10.0)
        .padded_w_of(ids.footer, 150.0)
        .top_left_with_margin_on(ids.footer, 10.0)
        .scrollbar_thickness(50.0)
        .set(ids.footer_listselect_view, ui);
    if let Some(s) = scrollbar {
        s.set(ui);
    }
    if let (Some(&SupportIdType::ImageId(_back_image)),
            Some(&SupportIdType::ImageId(arrows_image)),
            Some(&SupportIdType::ImageId(cloudy)),
            Some(&SupportIdType::ImageId(coin_info)),
            Some(&SupportIdType::ImageId(coin_info270)),
            Some(&SupportIdType::ImageId(icon_image))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::BACKCARD)),
         result_map.get(&ResourceEnum::Sprite(Sprite::ARROWS)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270)),
         result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {

        let (_l, _t, _r, _b, _c) = graphics_match::all_arrows(arrows_image);
        let mut buy_selected_id: Option<widget::Id> = None;
        while let Some(event) = events.next(ui, |i| {
            let mut y = false;
            if let &mut Some(_x) = buyselected {
                if _x == i {
                    y = true;
                }
            }
            y
        }) {
            use conrod_core::widget::list_select::Event;
            match event {
                // For the `Item` events we instantiate the `List`'s items.
                Event::Item(item) => {
                    let &(card_index, _, _, _,_, _, _, _,_) = handvec.get(item.i).unwrap();
                    let (_timeless, _string, _color, _app_font, _rect, _top_lefticon_rect) =
                        in_game::get_tile_image_withcost(card_index.clone(),
                                                         cardmeta,
                                                         appdata,
                                                         result_map);

                    let mut j = buy_list_item::ItemWidget::new(_timeless,
                                                               _string,
                                                               _rect,
                                                               _top_lefticon_rect,
                                                               "timeless")
                            .game_icon(icon_image)
                            .cloudy_image(cloudy)
                            .coin_info(coin_info)
                            .coin_info270(coin_info270)
                            .border_color(color::YELLOW)
                            .border(15.0)
                            .alphabet_font_id(_app_font)
                            .color(_color);
                    if let &mut Some(_s) = buyselected {
                        if _s == item.i {
                            buy_selected_id = Some(item.widget_id);
                            j = j.bordered();
                        }
                    }
                    item.set(j, ui);
                }
                Event::Selection(selected_id) => {
                    if let &mut Some(_s) = buyselected {
                        if _s == selected_id {
                            buy_selected_id = None;
                            *buyselected = None;
                        } else {
                            *buyselected = Some(selected_id);
                        }
                    } else {
                        *buyselected = Some(selected_id);
                    }
                }
                _ => {}
            }
        }
        if let (Some(_buy_selected_id), &mut Some(_buy_selected)) = (buy_selected_id, buyselected) {
            let j = ImageHover::new(_c)
                .w_h(item_h * 0.25, item_h * 0.25)
                .mid_right_with_margin_on(_buy_selected_id, -2.0)
                .set(ids.corner_arrow, ui);
            if let &mut Some(mut _overlay_blowup) = overlay_blowup {
                let (j, _, _, _, _, _, _,_,_) = handvec.get(_buy_selected).unwrap().clone();
                if j != _overlay_blowup {
                    _overlay_blowup = j;
                }
            }
            for _c in j {
                let (j, _, _, _, _, _, _,_,_) = handvec.get(_buy_selected).unwrap().clone();
                *overlay_blowup = Some(j);
            }

        }

        let exit_door = if *overlay { 9.0 } else { 8.0 };
        for _ in widget::Button::image(icon_image)
                .source_rectangle(graphics_match::gameicons_rect(exit_door))
                .w_h(80.0, 80.0)
                .right_from(ids.footerdragdroplistview, 0.0)
                .set(ids.footer_overlay_but, ui) {
            *overlay = true;
        }
        //chat button
        for _ in widget::Button::image(icon_image)
                .source_rectangle(graphics_match::gameicons_rect(11.0))
                .w_h(80.0, 80.0)
                .right_from(ids.footer_overlay_but, 0.0)
                .set(ids.footer_overlay_but2, ui) {
            *overlay_chat = true;
        }
        //exit button
        for _ in widget::Button::image(icon_image)
                .source_rectangle(graphics_match::gameicons_rect(10.0))
                .w_h(80.0, 80.0)
                .right_from(ids.footer_overlay_but2, 0.0)
                .set(ids.footer_overlay_but3, ui) {
            *overlay_exit = true;
        }
        //human button
        for _ in widget::Button::image(icon_image)
                .source_rectangle(graphics_match::gameicons_rect(12.0))
                .w_h(80.0, 80.0)
                .down_from(ids.footer_overlay_but, 0.0)
                .set(ids.footer_overlay_but4, ui) {
            *overlay_human = true;
        }
    }

}
fn spell(ui: &mut conrod_core::UiCell,
         ids: &Ids,
         personal: &mut Option<Personal>,
         appdata: &AppData,
         cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
         spell_which_arrangelist: &mut Option<widget::Id>,
         overlay_blowup: &mut Option<usize>,
         overlay: &mut bool,
         overlay_chat: &mut bool,
         overlay_exit: &mut bool,
         overlay_human: &mut bool,
         last_send: &mut Option<Instant>,
         result_map: &HashMap<ResourceEnum, SupportIdType>) {
    if let &mut Some(ref mut _personal) = personal {
        let temp = (*_personal).clone();
        let mut handvec =
            _personal.hand
                .clone()
                .iter()
                .map(|ref x| {
                         let (_timeless, _string, _color, _app_font, _rect, _top_left_rect) =
                        in_game::get_tile_image_withcost(*x.clone(), cardmeta, appdata, result_map);
                         (*x.clone(), _timeless, _string.to_owned(),None, _color, _app_font, _rect, _top_left_rect,false)
                     })
                .collect::<Vec<ArrangeTuple>>();
        if let (Some(&SupportIdType::ImageId(spinner_image)),
                Some(&SupportIdType::ImageId(back_image)),
                Some(&SupportIdType::ImageId(arrows_image)),
                Some(&SupportIdType::ImageId(cloudy)),
                Some(&SupportIdType::ImageId(coin_info)),
                Some(&SupportIdType::ImageId(coin_info270)),
                Some(&SupportIdType::ImageId(icon_image))) =
            (result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
             result_map.get(&ResourceEnum::Sprite(Sprite::BACKCARD)),
             result_map.get(&ResourceEnum::Sprite(Sprite::ARROWS)),
             result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
             result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
             result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270)),
             result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
            let spinner_rect = graphics_match::spinner_sprite();
            let (_l, _t, _r, _b, _c) = graphics_match::all_arrows(arrows_image);
            let footer_list_w = ui.w_of(ids.footer).unwrap() - 300.0;
            let (exitid, exitby, scrollbar,_) =
                ArrangeList::new(&mut handvec,
                                 spell_which_arrangelist,
                                 overlay_blowup,
                                 Box::new(move |tuple,_| {
                    let mut tuple1=tuple.clone();
                    tuple1.3 =None;                                     
                    ItemWidget::new(back_image,
                                    tuple1,
                                    "timeless".to_owned(),None)
                            .cloudy_image(cloudy)
                            .game_icon(icon_image)
                            .coin_info(coin_info)
                            .coin_info270(coin_info270)
                            .spinner_image(spinner_image, spinner_rect)
                            .border_color(color::YELLOW)
                            .border(15.0)
                            .toggle(false)
                }),
                                 Box::new(|(_v_index,
                                            _timelessbool,
                                            _string,
                                            _op_string,
                                            _color,
                                            _font,
                                            _rect,
                                            _top_left_rect,
                                            _inked)| {
                                              _v_index.clone()
                                          }),
                                 footer_list_w / 7.0)
                        .padded_h_of(ids.footer, 10.0)
                        .padded_w_of(ids.footer, 150.0)
                        .top_left_with_margin_on(ids.footer, 10.0)
                        .corner_arrow(_c)
                        .left_arrow(_l)
                        .right_arrow(_r)
                        .top_arrow(_t)
                        .arrow_size(appdata.convert_h(50.0))
                        .keypad_bool(false)
                        .set(ids.footerdragdroplistview, ui);
            match (exitid, exitby) {                
                (Some(_x), ExitBy::Top) => {
                    _personal.arranged.push((_x.0, false, None, false));
                }
                _ => {}
            }
            if let Some(s) = scrollbar {
                s.set(ui);
            }
            _personal.hand =
                handvec.iter().map(|&(x_index, _, _, _,_, _, _, _,_)| x_index).collect::<Vec<usize>>();
            if (*_personal).clone() != temp {
                println!("diff in hand");
                let now = Instant::now();
                *last_send = Some(now);
                let promptsender = PromptSender();
                let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
                let mut g = GameCommand::new();
                g.personal = Some(_personal.clone());
                h.set_gamecommand(g);
                promptsender.send(ServerReceivedMsg::serialize_send(h).unwrap());
            }
            let exit_door = if *overlay { 9.0 } else { 8.0 };
            for _ in widget::Button::image(icon_image)
                    .source_rectangle(graphics_match::gameicons_rect(exit_door))
                    .w_h(80.0, 80.0)
                    .right_from(ids.footerdragdroplistview, 0.0)
                    .set(ids.footer_overlay_but, ui) {
                *overlay = true;
            }
            //chat button
            for _ in widget::Button::image(icon_image)
                    .source_rectangle(graphics_match::gameicons_rect(11.0))
                    .w_h(80.0, 80.0)
                    .right_from(ids.footer_overlay_but, 0.0)
                    .set(ids.footer_overlay_but2, ui) {
                *overlay_chat = true;
            }
            //exit button
            for _ in widget::Button::image(icon_image)
                    .source_rectangle(graphics_match::gameicons_rect(10.0))
                    .w_h(80.0, 80.0)
                    .right_from(ids.footer_overlay_but2, 0.0)
                    .set(ids.footer_overlay_but3, ui) {
                *overlay_exit = true;
            }
            //human button
            for _ in widget::Button::image(icon_image)
                    .source_rectangle(graphics_match::gameicons_rect(12.0))
                    .w_h(80.0, 80.0)
                    .down_from(ids.footer_overlay_but, 0.0)
                    .set(ids.footer_overlay_but4, ui) {
                *overlay_human = true;
            }
        }
    }


}
fn show_draft(ui: &mut conrod_core::UiCell,
              ids: &Ids,
              print_instruction_set: &mut Vec<bool>,
              print_instruction_cache: &mut usize,
              app: &AppData) {
    let g_vec = (*app)
        .texts
        .instructions1
        .iter()
        .enumerate()
        .filter(|&(index, _)| if index < 3 { true } else { false })
        .zip((*app).texts.instructions2.iter())
        .map(|((_index, ref label), &(ref rect_tuple, ref oval_option))| {
                 Instruction(label, rect_tuple, oval_option, ids.footer)
             })
        .collect::<Vec<Instruction>>();
    if let Some(_pi) = print_instruction_set.get_mut(0) {
        if *_pi {
            *_pi = InstructionSet::new(&g_vec, print_instruction_cache, (*app).texts.next)
                .parent_id(ids.footer)
                .label_color(color::WHITE)
                .set(ids.instructionview, ui);
        }
    }

}

fn buy(ui: &mut conrod_core::UiCell,
       ids: &Ids,
       _player: &mut Player,
       overlay2: &mut bool,
       buyselected: &mut Option<usize>,
       appdata: &AppData,
       result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let text = if buyselected.is_some() {
        appdata.texts.buy
    } else {
        appdata.texts.continue_without_buying
    };
    if let Some(_) = widget::Button::new()
           .label(&text)
           .mid_top_of(ids.footer)
           .w_h(200.0, 80.0)
           .set(ids.submit_but, ui)
           .next() {
        let promptsender = PromptSender();
        let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
        let mut g = GameCommand::new();
        g.buy_offer = Some((buyselected.is_some(), buyselected.unwrap_or(0)));
        h.set_gamecommand(g);
        promptsender.clone().send(ServerReceivedMsg::serialize_send(h).unwrap());
    }
    if let Some(&SupportIdType::ImageId(icon_image)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS)) {
        let default_color = color::GREY;
        let icon_v = graphics_match::gameicons_listitem(icon_image,
                                                        _player.ink.clone(),
                                                        _player.remover.clone(),
                                                        _player.coin.clone(),
                                                        _player.literacy_award.clone(),
                                                        _player.vp.clone(),
                                                        _player.draftlen.clone());

        let slist = List::new(icon_v.clone(), overlay2)
            .color(default_color)
            .label("Player Info")
            .label_color(default_color.plain_contrast())
            .down_from(ids.submit_but, 0.0)
            .align_middle_x_of(ids.submit_but)
            .h(80.0)
            .w_of(ids.footer)
            .set(ids.overlay_player_info, ui);

        if let (Some(_s), Some(_si), Some(xy)) = slist {
            let _dim = [300.0, 100.0];
            animated_canvas::Canvas::new()
                .x(xy[0] + _dim[0] * 0.5)
                .y(-150.0)
                .parent(ids.master)
                .color(default_color)
                .wh(_dim)
                .set(ids.overlay2_canvas, ui);
            if let Some(&IconStruct(ref _image, _, ref _desc)) = icon_v.get(_s) {
                _image.wh([20.0, 20.0]).mid_left_of(ids.overlay2_canvas).set(ids.overlay2_image,
                                                                             ui);
                let fontsize = get_font_size_hn(_dim[1], 4.0);
                widget::Text::new(&_desc)
                    .font_size(fontsize)
                    .color(default_color.plain_contrast())
                    .align_middle_y_of(ids.overlay2_image)
                    .right_from(ids.overlay2_image, 0.0)
                    .w(_dim[0] - 20.0)
                    .h(_dim[1])
                    .set(ids.overlay2_text, ui);
            }
        }
    }


}
fn trash_other(ui: &mut conrod_core::UiCell,
               ids: &Ids,
               _player: &mut Player,
               overlay2: &mut bool,
               buyselected: &mut Option<usize>,
               appdata: &AppData,
               result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let text = if buyselected.is_some() {
        appdata.texts.trash_other
    } else {
        appdata.texts.continue_without_trashing_other
    };
    if let Some(_) = widget::Button::new()
           .label(&text)
           .mid_top_of(ids.footer)
           .w_h(200.0, 80.0)
           .set(ids.submit_but, ui)
           .next() {
        let promptsender = PromptSender();
        let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
        let mut g = GameCommand::new();
        g.buy_offer = Some((buyselected.is_some(), buyselected.unwrap_or(0)));
        h.set_gamecommand(g);
        promptsender.clone().send(ServerReceivedMsg::serialize_send(h).unwrap());
    }
    if let Some(&SupportIdType::ImageId(icon_image)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS)) {
        let default_color = color::GREY;
        let icon_v = graphics_match::gameicons_listitem(icon_image,
                                                        _player.ink.clone(),
                                                        _player.remover.clone(),
                                                        _player.coin.clone(),
                                                        _player.literacy_award.clone(),
                                                        _player.vp.clone(),
                                                        _player.draftlen.clone());

        let slist = List::new(icon_v.clone(), overlay2)
            .color(default_color)
            .label("Player Info")
            .label_color(default_color.plain_contrast())
            .down_from(ids.submit_but, 0.0)
            .align_middle_x_of(ids.submit_but)
            .h(80.0)
            .w_of(ids.footer)
            .set(ids.overlay_player_info, ui);

        if let (Some(_s), Some(_si), Some(xy)) = slist {
            let _dim = [300.0, 100.0];
            animated_canvas::Canvas::new()
                .x(xy[0])
                .y(-200.0)
                .parent(ids.master)
                .color(default_color)
                .wh(_dim)
                .set(ids.overlay2_canvas, ui);
            if let Some(&IconStruct(ref _image, _, ref _desc)) = icon_v.get(_s) {
                _image.wh([20.0, 20.0]).mid_left_of(ids.overlay2_canvas).set(ids.overlay2_image,
                                                                             ui);
                let fontsize = get_font_size_hn(_dim[1], 4.0);
                widget::Text::new(&_desc)
                    .font_size(fontsize)
                    .color(default_color.plain_contrast())
                    .align_middle_y_of(ids.overlay2_image)
                    .right_from(ids.overlay2_image, 0.0)
                    .w(_dim[0] - 20.0)
                    .h(_dim[1])
                    .set(ids.overlay2_text, ui);
            }

        }
    }


}
