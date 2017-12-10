use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable,
             Borderable, Rect};
use conrod::widget::primitive::image::Image;
use cardgame_widgets::custom_widget::image_hover::{Hoverable, ImageHover};
use cardgame_widgets::custom_widget::arrange_list::{ArrangeList, ExitBy};
use custom_widget::arrange_list_item::ItemWidget;
use cardgame_widgets::custom_widget::instructionset::InstructionSet;
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::custom_widget::promptview::PromptSender;
use cardgame_widgets::text::get_font_size_hn;
use cardgame_widgets::custom_widget::player_info::list::List;
use cardgame_widgets::custom_widget::player_info::item::IconStruct;
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::{cards, local};
use graphics_match;
use graphics_match::ImageHoverable;
use logic::in_game;
use instruction::Instruction;
use logic::body::PromptSendable;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              _action_tx: mpsc::Sender<OwnedMessage>) {
    let GameData { ref page_index,
                   ref mut boardcodec,
                   ref mut print_instruction_set,
                   ref mut personal,
                   ref mut overlay,
                   ref mut overlay2,
                   ref mut overlay_chat,
                   ref mut overlay_exit,
                   ref mut buy_selected,
                   .. } = *gamedata;
    if let &mut Some(ref mut boardcodec) = boardcodec {
        let card_images = in_game::card_images(result_map);
        if let Some(ref mut _player) = boardcodec.players.get_mut(*page_index) {
            match gamedata.guistate {
                app::GuiState::Game(GameState::ShowDraft) => {
                    show_draft(ui, ids, print_instruction_set, &appdata);
                }
                app::GuiState::Game(GameState::Spell) => {
                    spell(ui,
                          ids,
                          _player,
                          &card_images,
                          &appdata,
                          personal,
                          overlay,
                          overlay_chat,
                          overlay_exit,
                          result_map,
                          _action_tx);
                }
                app::GuiState::Game(GameState::TurnToSubmit) => {
                    spell(ui,
                          ids,
                          _player,
                          &card_images,
                          &appdata,
                          personal,
                          overlay,
                          overlay_chat,
                          overlay_exit,
                          result_map,
                          _action_tx);
                }
                app::GuiState::Game(GameState::Buy) => {
                    buy(ui,
                        ids,
                        _player,
                        overlay2,
                        buy_selected,
                        appdata,
                        result_map,
                        _action_tx.clone());
                }
                app::GuiState::Game(GameState::TrashOther(_)) => {
                    trash_other(ui,
                                ids,
                                _player,
                                overlay2,
                                buy_selected,
                                appdata,
                                result_map,
                                _action_tx.clone());
                }
                _ => {}
            }
        }
    }

    //  draw_hand(ui, ids, gamedata, appdata, result_map);
}
fn spell(ui: &mut conrod::UiCell,
         ids: &Ids,
         _player: &mut Player,
         card_images: &[Option<image::Id>; 27],
         appdata: &AppData,
         personal: &mut Option<Personal>,
         overlay: &mut bool,
         overlay_chat: &mut bool,
         overlay_exit: &mut bool,
         result_map: &HashMap<ResourceEnum, SupportIdType>,
         _action_tx: mpsc::Sender<OwnedMessage>) {
    if let &mut Some(ref mut _personal) = personal {
        let temp = (*_personal).clone();
        let mut handvec = _personal.hand
            .clone()
            .iter()
            .map(|ref x| {
                let (_image_id, _rect, _theme) =
                    in_game::get_card_widget_image_portrait(x.clone().clone(),
                                                            card_images,
                                                            appdata);
                (x.clone().clone(), _image_id, _rect)
            })
            .collect::<Vec<(usize, image::Id, Rect)>>();
        if let (Some(&SupportIdType::ImageId(spinner_image)),
                Some(&SupportIdType::ImageId(back_image)),
                Some(&SupportIdType::ImageId(arrows_image)),
                Some(&SupportIdType::ImageId(icon_image))) =
            (result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
             result_map.get(&ResourceEnum::Sprite(Sprite::BACKCARD)),
             result_map.get(&ResourceEnum::Sprite(Sprite::ARROWS)),
             result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
            let spinner_rect = graphics_match::spinner_sprite();
            let (_l, _t, _r, _b) = graphics_match::all_arrows(arrows_image);
            let (exitid, exitby, scrollbar) =
                ArrangeList::new(&mut handvec,
                                 Box::new(move |(_v_index, v_blowup, v_rect)| {
                    let i_h_struct =
                        ImageHoverable(Image::new(v_blowup).source_rectangle(v_rect), Some(Image::new(v_blowup).source_rectangle(graphics_match::cards_btm(v_rect))), Some(Image::new(v_blowup).source_rectangle(graphics_match::cards_btm(v_rect))));
                    let t_i_h_struct =
                        ImageHoverable(Image::new(back_image.clone())
                                           .source_rectangle(graphics_match::backcard()),
                                       None,
                                       None);
                    ItemWidget::new(i_h_struct, t_i_h_struct)
                        .spinner_image(spinner_image, spinner_rect)
                        .border_color(color::YELLOW)
                        .border(20.0)
                }),
                                 200.0)
                        .padded_h_of(ids.footer, 10.0)
                        .padded_w_of(ids.footer, 150.0)
                        .top_left_with_margin_on(ids.footer, 10.0)
                        .left_arrow(_l)
                        .right_arrow(_r)
                        .top_arrow(_t)
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
            _personal.hand = handvec.iter().map(|&(x_index, _, _)| x_index).collect::<Vec<usize>>();
            if (*_personal).clone() != temp {
                let promptsender = PromptSendable(_action_tx);
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
        }
    }


}
fn show_draft(ui: &mut conrod::UiCell,
              ids: &Ids,
              print_instruction_set: &mut Vec<bool>,
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
            *_pi = InstructionSet::new(&g_vec, (*app).texts.next)
                .parent_id(ids.footer)
                .label_color(color::WHITE)
                .set(ids.instructionview, ui);
        }
    }

}


#[allow(dead_code)]
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
#[allow(dead_code)]
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
fn buy(ui: &mut conrod::UiCell,
       ids: &Ids,
       _player: &mut Player,
       overlay2: &mut bool,
       buyselected: &mut Option<usize>,
       appdata: &AppData,
       result_map: &HashMap<ResourceEnum, SupportIdType>,
       _action_tx: mpsc::Sender<OwnedMessage>) {
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
        let promptsender = PromptSendable(_action_tx.clone());
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
fn trash_other(ui: &mut conrod::UiCell,
               ids: &Ids,
               _player: &mut Player,
               overlay2: &mut bool,
               buyselected: &mut Option<usize>,
               appdata: &AppData,
               result_map: &HashMap<ResourceEnum, SupportIdType>,
               _action_tx: mpsc::Sender<OwnedMessage>) {
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
        let promptsender = PromptSendable(_action_tx.clone());
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
