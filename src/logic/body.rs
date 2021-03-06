use conrod_core::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable,
             Borderable, Rect, text, Color};
use cardgame_widgets::custom_widget::bordered_image::Bordered;
use conrod_core::widget::primitive::image::Image;
use conrod_core::widget::envelope_editor::EnvelopePoint;
use cardgame_widgets::custom_widget::image_hover::{Hoverable, ImageHover};
use cardgame_widgets::custom_widget::arrange_list::{ArrangeList, ExitBy};
use custom_widget::arrange_list_tile::{ArrangeTuple,ItemWidget};
use custom_widget::buy_list_item;
use custom_widget::show_draft_item;
use cardgame_widgets::custom_widget::shuffle::Shuffle;
use cardgame_widgets::custom_widget::promptview::{PromptView, PromptSendable};
use cardgame_widgets::custom_widget::instructionset::InstructionSet;
use cardgame_widgets::custom_widget::player_info; //player_info::list::List,//::item::IconStruct
use cardgame_widgets::sprite::spriteable_rect;
use backend::codec_lib::codec::*;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta;
use std::collections::HashMap;
use std;
use std::time::Instant;
use app::{self, GameData, Ids, GuiState};
use logic::in_game;
use logic;
use graphics_match;
use graphics_match::ImageHoverable;
use app::{BoardStruct, PromptSender};
use backend::codec_lib;
//
pub fn render(ui: &mut conrod_core::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let GameData { ref page_index,
                   ref mut boardcodec,
                   ref mut print_instruction_set,
                   ref mut guistate,
                   ref mut initial_draft,
                   ref mut overlay_blowup,
                   ref player_index,
                   ref notification,
                   ref mut personal,
                   ref mut buy_selected,
                   ref mut overlay2,
                   ref mut last_send,
                   ref mut spell_which_arrangelist,
                   ref mut keypad_on,
                   .. } = *gamedata;
    if let &mut Some(ref mut boardcodec) = boardcodec {
        if let (Some(ref mut _player), ref offer_row) =
            (boardcodec.players.get_mut(*page_index), &boardcodec.offer_row) {
            match guistate {
                &mut app::GuiState::Game(GameState::ShowDraft) => {
                    show_draft(ui,
                               ids,
                               _player,
                               &cardmeta,
                               &appdata,
                               print_instruction_set,
                               initial_draft,
                               result_map);
                }
                &mut app::GuiState::Game(GameState::Shuffle) => {
                    shuffle(ui,
                            ids,
                            _player,
                            &cardmeta,
                            &appdata,
                            &initial_draft,
                            player_index,
                            guistate,
                            personal,
                            result_map);
                }
                &mut app::GuiState::Game(GameState::Spell) => {
                    if player_index.unwrap() != *page_index {
                        view_others(ui,
                                    ids,
                                    &cardmeta,
                                    _player.clone(),
                                    spell_which_arrangelist,
                                    overlay_blowup,
                                    buy_selected,
                                    appdata,
                                    result_map);
                    } else {
                        spell(ui,
                              ids,
                              &cardmeta,
                              personal,
                              spell_which_arrangelist,
                              overlay_blowup,
                              last_send,
                              keypad_on,
                              appdata,
                              result_map);
                    }
                }
                &mut app::GuiState::Game(GameState::TurnToSubmit) => {
                    spell(ui,
                          ids,
                          &cardmeta,
                          personal,
                          spell_which_arrangelist,
                          overlay_blowup,
                          last_send,
                          keypad_on,
                          appdata,
                          result_map);
                    if let Some(&SupportIdType::ImageId(spinner_image)) =
                        result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)) {
                        turn_to_submit_but(ui,
                                           ids,
                                           &appdata,
                                           overlay_blowup,
                                           last_send.clone(),
                                           spinner_image);
                    }
                }
                &mut app::GuiState::Game(GameState::Buy) => {
                    buy(ui,
                        ids,
                        &cardmeta,
                        offer_row,
                        buy_selected,
                        overlay_blowup,
                        appdata,
                        result_map);
                }
                &mut app::GuiState::Game(GameState::TrashOther(_otherthanthis)) => {
                    trash_other(ui,
                                ids,
                                _player,
                                _otherthanthis,
                                &cardmeta,
                                buy_selected,
                                appdata,
                                result_map);
                }

                _ => {}
            }

        }
        match guistate {
            &mut app::GuiState::Game(GameState::ShowResult(_w)) => {
                show_result(ui,
                            ids,
                            &boardcodec.players,
                            _w,
                            overlay2,
                            &appdata,
                            result_map);
            }
            _ => {}
        }
        logic::notification::render(ui, ids, ids.body, notification.clone());
    }

    //  draw_hand(ui, ids, gamedata, appdata, result_map);
}
fn turn_to_submit_but(ui: &mut conrod_core::UiCell,
                      ids: &Ids,
                      appdata: &AppData,
                      overlay_blowup: &mut Option<usize>,
                      last_send: Option<Instant>,
                      spinner_image: image::Id) {
    let spinner_rect = graphics_match::spinner_sprite();
    let promptsender = PromptSender();
    if let Some(_last_send) = last_send {
        let ratio = _last_send.elapsed()
            .checked_div(30_000_000)
            .unwrap()
            .subsec_nanos() as f64;
        let spinner_index = ((ratio / 10.0).floor() as f64) / 10.0 * 60.0;
        let _rect = spriteable_rect(spinner_rect, spinner_index as f64);
        widget::Image::new(spinner_image)
            .source_rectangle(Rect::from_corners(_rect.0, _rect.1))
            .wh(appdata.convert_dim([50.0, 50.0]))
            .mid_bottom_of(ids.body)
            .set(ids.loading_gif, ui);
    } else {
        if let Some(_) = widget::Button::new()
               .label(&appdata.texts.submit)
               .mid_bottom_of(ids.body)
               .wh(appdata.convert_dim([100.0, 50.0]))
               .set(ids.submit_but, ui)
               .next() {
            *overlay_blowup = None;
            println!("submit word");
            let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
            let mut g = GameCommand::new();
            g.submit_word = Some(true);
            h.set_gamecommand(g);
            promptsender.clone().send(ServerReceivedMsg::serialize_send(h).unwrap());
        }
    }

}
fn show_draft(ui: &mut conrod_core::UiCell,
              ids: &Ids,
              player: &mut Player,
              cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
              appdata: &AppData,
              print_instruction_set: &mut Vec<bool>,
              initial_draft: &mut Vec<usize>,
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let body_w = ui.w_of(ids.body).unwrap();
    let item_h = body_w / 10.0;
    *initial_draft = player.draft.clone();
    let mut dealt_iter = player.draft.iter();
    if let (Some(&SupportIdType::ImageId(cloudy)),
            Some(&SupportIdType::ImageId(coin_info)),
            Some(&SupportIdType::ImageId(coin_info270)),
            Some(&SupportIdType::ImageId(game_icon))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270)),
         result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
        if let Some(&mut true) = print_instruction_set.get_mut(0) {
            let (mut items, scrollbar) = widget::List::flow_right(player.draft.len())
                .item_size(item_h)
                .instantiate_all_items()
                .mid_bottom_of(ids.body)
                .h(item_h * 1.2)
                .padded_w_of(ids.body, 20.0)
                .scrollbar_next_to()
                .set(ids.listview, ui);
            if let Some(s) = scrollbar {
                s.set(ui)
            }
            while let (Some(item), Some(card_index)) = (items.next(ui), dealt_iter.next()) {
                let (_timeless, _string, _color, _font, _rect, _top_lefticon_rect) =
                    in_game::get_tile_image_withcost(card_index.clone(),
                                                     cardmeta,
                                                     appdata,
                                                     result_map);
                let j = show_draft_item::ItemWidget::new(_timeless,
                                                         _string,
                                                         _rect,
                                                         _top_lefticon_rect,
                                                         "timeless")
                        .cloudy_image(cloudy)
                        .coin_info(coin_info)
                        .coin_info270(coin_info270)
                        .game_icon(game_icon)
                        .alphabet_font_id(_font)
                        .color(_color);
                item.set(j, ui);
            }
        } else {

            let promptsender = PromptSender();
            let instructions: Vec<(String, Box<Fn(PromptSender)>)> = vec![("Continue".to_owned(),
                                                                           Box::new(move |ps| {
                let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
                let mut g = GameCommand::new();
                g.go_to_shuffle = Some(true);
                h.set_gamecommand(g);
                ps.send(ServerReceivedMsg::serialize_send(h).unwrap());
            }))];
            
            let mut prompt =
                Some((0.5f64, "Lets' start to Shuffle the cards".to_owned(), instructions));
            let prompt_j = PromptView::new(&mut prompt, promptsender)
                .wh_of(ids.master)
                .color(color::LIGHT_GREY)
                .middle_of(ids.master);
            prompt_j.set(ids.promptview, ui);
            
        }
    }
}
fn shuffle(ui: &mut conrod_core::UiCell,
           ids: &Ids,
           player: &Player,
           cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
           appdata: &AppData,
           initial_draft: &Vec<usize>,
           player_index: &Option<usize>,
           guistate: &mut GuiState,
           personal: &mut Option<Personal>,
           result_map: &HashMap<ResourceEnum, SupportIdType>) {
    if let (Some(&SupportIdType::ImageId(back_logo)),
            Some(&SupportIdType::ImageId(cloudy)),
            Some(&SupportIdType::ImageId(coin_info)),
            Some(&SupportIdType::ImageId(coin_info270)),
            Some(&SupportIdType::ImageId(spinner_image)),
            Some(&SupportIdType::ImageId(game_icon)),
            &Some(_player_index)) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::BACKCARD)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270)),
         result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
         result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS)),
         player_index) {
        let card_vec =
            initial_draft.iter()
                .map(|x| {
                    let (_timeless, _string, _color, _app_font, _rect, _top_lefticon_rect) =
                        in_game::get_tile_image_withcost(x.clone(), cardmeta, appdata, result_map);
                    (x.clone(), _timeless, _string.to_owned(),None, _color, _app_font, _rect, _top_lefticon_rect,false)
                })
                .collect::<Vec<ArrangeTuple>>();
        let give_out_vec = player.hand
            .iter()
            .enumerate()
            .map(move |(_index, x)| {
                let mut _z = None;
                for (ref _initial_index, _d) in initial_draft.clone().iter().enumerate() {
                    if _d == x {
                        _z = Some(_initial_index.clone());
                    }
                }
                _z
            })
            .filter(|x| if let &Some(_) = x { true } else { false })
            .map(|x| x.unwrap())
            .collect::<Vec<usize>>();
        if !Shuffle::new(&card_vec,
                         Box::new(move |tuple| {
            let spinner_rect = graphics_match::spinner_sprite();
            ItemWidget::new(back_logo,
                            tuple,
                            "timeless".to_owned(),None)
                    .cloudy_image(cloudy)
                    .game_icon(game_icon)
                    .coin_info(coin_info)
                    .coin_info270(coin_info270)
                    .spinner_image(spinner_image, spinner_rect)
                    .border_color(color::YELLOW)
                    .border(15.0)
                    .toggle(false)
        }),
                         Image::new(back_logo))
                    .give_out(give_out_vec)
                    .bottom_left_of(ids.body)
                    .h(appdata.convert_h(140.0))
                    .w_of(ids.body)
                    .image_dim(appdata.convert_dim([110.0, 140.0]))
                    .close_frame_rate(25)
                    .set(ids.shuffleview, ui) {
            if _player_index == 0 {
                *guistate = GuiState::Game(GameState::TurnToSubmit);
            } else {
                *guistate = GuiState::Game(GameState::Spell);
            }
            cache_personal(player, personal);
        }
    }
}
fn cache_personal(player: &Player, personal: &mut Option<Personal>) {
    if let &mut None = personal {
        *personal = Some(Personal {
                             hand: player.hand.clone(),
                             arranged: vec![],
                         });
    }
}

fn spell(ui: &mut conrod_core::UiCell,
         ids: &Ids,
         cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
         personal: &mut Option<Personal>,
         spell_which_arrangelist: &mut Option<widget::Id>,
         overlay_blowup: &mut Option<usize>,
         last_send: &mut Option<Instant>,
         keypad_bool:&mut bool,
         appdata: &AppData,
         result_map: &HashMap<ResourceEnum, SupportIdType>) {
    if let &mut Some(ref mut _personal) = personal {
        let temp = (*_personal).clone();
        let mut arrangedvec = _personal.arranged
            .clone()
            .iter()
            .map(|&(ref x, ref ink, ref op_string, ref _timeless)| {
                let (_timeless, _string, _color, _font, _rect, _top_lefticon_rect) =
                    in_game::get_tile_image_withcost(x.clone(), cardmeta, appdata, result_map);
                (x.clone(),
                 _timeless,
                 _string.to_owned(),
                 op_string.clone(),
                 _color,
                 _font,
                 _rect,
                 _top_lefticon_rect,
                 ink.clone())
            })
            .collect::<Vec<ArrangeTuple>>();
        if let (Some(&SupportIdType::ImageId(spinner_image)),
                Some(&SupportIdType::ImageId(back_image)),
                Some(&SupportIdType::ImageId(arrows_image)),
                Some(&SupportIdType::ImageId(cloudy)),
                Some(&SupportIdType::ImageId(coin_info)),
                Some(&SupportIdType::ImageId(coin_info270)),
                Some(&SupportIdType::ImageId(game_icon))) =
            (result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
             result_map.get(&ResourceEnum::Sprite(Sprite::BACKCARD)),
             result_map.get(&ResourceEnum::Sprite(Sprite::ARROWS)),
             result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
             result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
             result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270)),
             result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
            let spinner_rect = graphics_match::spinner_sprite();
            let (_l, _t, _r, _b, _c) = graphics_match::all_arrows(arrows_image);
            let body_list_w = ui.w_of(ids.body).unwrap() - 40.0;
            let (exitid, exitby, scrollbar,keypad_bool_new) =
                ArrangeList::new(&mut arrangedvec,
                                 spell_which_arrangelist,
                                 overlay_blowup,
                                 Box::new(|tuple,keypad_bool_ind| {
                    ItemWidget::new(back_image,
                                    tuple,
                                    "timeless".to_owned(),
                                    Some((appdata,result_map,keypad_bool_ind,ids.body)))
                            .cloudy_image(cloudy)
                            .game_icon(game_icon)
                            .coin_info(coin_info)
                            .coin_info270(coin_info270)
                            .spinner_image(spinner_image, spinner_rect)
                            .border_color(color::YELLOW)
                            .border(15.0)
                            .toggle(true)
                }),
                                 Box::new(|(_v_index,
                                            _timelessbool,
                                            _string,
                                            _op_string,
                                            _color,
                                            _font,
                                            _rect,
                                            _top_left_rect,
                                            _ink
                                            )| {
                                              _v_index.clone()
                                          }),
                                 body_list_w / 7.0)
                        .h(appdata.convert_h(210.0))
                        .padded_w_of(ids.body, 20.0)
                        .mid_bottom_with_margin_on(ids.body, appdata.convert_h(50.0))
                        .left_arrow(_l)
                        .right_arrow(_r)
                        .bottom_arrow(_b)
                        .corner_arrow(_c)
                        .arrow_size(appdata.convert_h(50.0))
                        .keypad_bool(keypad_bool.clone())
                        .set(ids.bodydragdroplistview, ui);
            *keypad_bool = keypad_bool_new;
            match (exitid, exitby) {                
                (Some(_x), ExitBy::Bottom) => {
                    _personal.hand.push(_x.0);
                }
                _ => {}
            }
            if let Some(s) = scrollbar {
                s.set(ui);
            }

            _personal.arranged = arrangedvec.iter()
                .map(|&(ref x_index,
                        ref _timeless,
                        ref _string,
                        ref op_string,
                        ref _color,
                        ref _font,
                        ref _rect,
                        ref _top_lefticon_rect,
                        ref _ink)| {
                         (x_index.clone(), _ink.clone(), op_string.clone(), _timeless.clone())
                     })
                .collect::<Vec<(usize, bool, Option<String>, bool)>>();

            if (*_personal).clone() != temp {
                println!("diff");
                let now = Instant::now();
                *last_send = Some(now);
                let promptsender = PromptSender();
                let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
                let mut g = GameCommand::new();
                g.personal = Some(_personal.clone());
                h.set_gamecommand(g);
                promptsender.send(ServerReceivedMsg::serialize_send(h).unwrap());
            }
        }
    }

}
fn view_others(ui: &mut conrod_core::UiCell,
               ids: &Ids,
               cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
               player: Player,
               spell_which_arrangelist: &mut Option<widget::Id>,
               overlay_blowup: &mut Option<usize>,
               buyselected: &mut Option<usize>,
               appdata: &AppData,
               result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let arrangedvec = player.arranged
        .clone()
        .iter()
        .map(|&(ref x, ref ink, ref op_string, ref _timeless)| {
            let (_timeless, _string, _color, _font, _rect, _top_lefticon_rect) =
                in_game::get_tile_image_withcost(x.clone(), cardmeta, appdata, result_map);
            (x.clone(),
             _timeless,
             _string.to_owned(),
             op_string.clone(),
             _color,
             _font,
             _rect,
             _top_lefticon_rect,
             ink.clone())
        })
        .collect::<Vec<ArrangeTuple>>();
    let body_w = ui.w_of(ids.body).unwrap();
    let item_h = body_w / 7.0;
    let (mut events, scrollbar) = widget::ListSelect::single(arrangedvec.len())
        .flow_right()
        .item_size(item_h)
        .mid_bottom_of(ids.body)
        .h(item_h * 1.2)
        .padded_w_of(ids.body, 20.0)
        .scrollbar_thickness(10.0)
        .set(ids.listselect_view, ui);
    if let Some(s) = scrollbar {
        s.set(ui);
    }
    if let (Some(&SupportIdType::ImageId(spinner_image)),
            Some(&SupportIdType::ImageId(back_image)),
            Some(&SupportIdType::ImageId(arrows_image)),
            Some(&SupportIdType::ImageId(cloudy)),
            Some(&SupportIdType::ImageId(coin_info)),
            Some(&SupportIdType::ImageId(coin_info270)),
            Some(&SupportIdType::ImageId(_game_icon))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
         result_map.get(&ResourceEnum::Sprite(Sprite::BACKCARD)),
         result_map.get(&ResourceEnum::Sprite(Sprite::ARROWS)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270)),
         result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
        let spinner_rect = graphics_match::spinner_sprite();
        let (_l, _t, _r, _b, _c) = graphics_match::all_arrows(arrows_image);
        let body_list_w = ui.w_of(ids.body).unwrap() - 40.0;

        let mut buy_selected_id: Option<widget::Id> = None;
        let (_l, _t, _r, _b, _c) = graphics_match::all_arrows(arrows_image);
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
                    let &(card_index, _, _, _, _, _, _, _, _) = arrangedvec.get(item.i).unwrap();
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
                            .game_icon(_game_icon)
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
                let (j, _, _, _, _, _, _, _, _) = arrangedvec.get(_buy_selected).unwrap().clone();
                if j != _overlay_blowup {
                    _overlay_blowup = j;
                }
            }
            for _c in j {
                let (j, _, _, _, _, _, _, _, _) = arrangedvec.get(_buy_selected).unwrap().clone();
                *overlay_blowup = Some(j);
            }

        }

    }

}
fn buy(ui: &mut conrod_core::UiCell,
       ids: &Ids,
       cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
       offer_row: &Vec<usize>,
       buyselected: &mut Option<usize>,
       overlay_blowup: &mut Option<usize>,
       appdata: &AppData,
       result_map: &HashMap<ResourceEnum, SupportIdType>) {

    widget::Text::new(appdata.texts.buy)
        .color(color::WHITE)
        .font_size(40)
        .h(80.0)
        .w_of(ids.body)
        .top_left_of(ids.body)
        .set(ids.body_header_text, ui);
    widget::Text::new(appdata.texts.unused_coins)
        .color(color::GREY)
        .font_size(30)
        .padded_w_of(ids.body, 90.0)
        .h(60.0)
        .down_from(ids.body_header_text, 0.0)
        .set(ids.body_subject_text, ui);
    let body_w = ui.w_of(ids.body).unwrap();
    let item_h = body_w / 7.0;
    let (mut events, scrollbar) = widget::ListSelect::single(offer_row.len())
        .flow_right()
        .item_size(item_h)
        .mid_bottom_of(ids.body)
        .h(item_h * 1.2)
        .padded_w_of(ids.body, 20.0)
        .scrollbar_thickness(10.0)
        .set(ids.listselect_view, ui);
    if let Some(s) = scrollbar {
        s.set(ui)
    }
    if let (Some(&SupportIdType::ImageId(cloudy)),
            Some(&SupportIdType::ImageId(coin_info)),
            Some(&SupportIdType::ImageId(coin_info270)),
            Some(&SupportIdType::ImageId(arrows_image)),
            Some(&SupportIdType::ImageId(game_icon))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270)),
         result_map.get(&ResourceEnum::Sprite(Sprite::ARROWS)),
         result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
        let mut buy_selected_id: Option<widget::Id> = None;
        let (_l, _t, _r, _b, _c) = graphics_match::all_arrows(arrows_image);
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
                    let card_index = offer_row.get(item.i).unwrap();
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
                            .game_icon(game_icon)
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
                let j = offer_row.get(_buy_selected).unwrap().clone();
                if j != _overlay_blowup {
                    _overlay_blowup = j;
                }
            }
            for _c in j {
                let j = offer_row.get(_buy_selected).unwrap().clone();
                *overlay_blowup = Some(j);
            }

        }
    }
}
fn trash_other(ui: &mut conrod_core::UiCell,
               ids: &Ids,
               player: &Player,
               otherthanthis: usize,
               cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
               buyselected: &mut Option<usize>,
               appdata: &AppData,
               result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let mut hand = player.hand.clone();
    let arranged = player.arranged
        .iter()
        .map(|&(ref ci, _, _, ref time)| {
                 if (*time) | (*ci == otherthanthis) {
                     return None;
                 }
                 return Some(ci.clone());
             })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<usize>>();
    hand.extend(arranged);
    widget::Text::new(appdata.texts.trash)
        .color(color::WHITE)
        .font_size(60)
        .h(100.0)
        .w_of(ids.body)
        .top_left_of(ids.body)
        .set(ids.body_header_text, ui);
    widget::Text::new(appdata.texts.trash_other)
        .color(color::WHITE)
        .font_size(50)
        .h(80.0)
        .down_from(ids.body_header_text, 0.0)
        .set(ids.body_subject_text, ui);
    let body_w = ui.w_of(ids.body).unwrap();
    let item_h = body_w / 5.0;
    let (mut events, scrollbar) = widget::ListSelect::single(hand.len())
        .flow_right()
        .item_size(item_h)
        .mid_bottom_of(ids.body)
        .h(item_h * 1.2)
        .padded_w_of(ids.body, 20.0)
        .scrollbar_thickness(10.0)
        .set(ids.listselect_view, ui);
    if let Some(s) = scrollbar {
        s.set(ui)
    }
    if let (Some(&SupportIdType::ImageId(back_logo)),
            Some(&SupportIdType::ImageId(cloudy)),
            Some(&SupportIdType::ImageId(coin_info)),
            Some(&SupportIdType::ImageId(coin_info270)),
            Some(&SupportIdType::ImageId(spinner_image)),
            Some(&SupportIdType::ImageId(game_icon))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::BACKCARD)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CLOUDY)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO)),
         result_map.get(&ResourceEnum::Sprite(Sprite::COININFO270)),
         result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)),
         result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS))) {
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
                    let card_index = hand.get(item.i).unwrap();
                    let spinner_rect = graphics_match::spinner_sprite();
                    let (_timeless, _string, _color, _app_font, _rect, _top_lefticon_rect) =
                        in_game::get_tile_image_withcost(card_index.clone(),
                                                         cardmeta,
                                                         appdata,
                                                         result_map);                                            
                    let op_string =None;
                    let tuple = (card_index.clone(),_timeless,_string.to_owned(),op_string,_color,_app_font,_rect,_top_lefticon_rect,false);
                    let j = ItemWidget::new(back_logo,
                                            tuple,
                                            "timeless".to_owned(),None)
                            .cloudy_image(cloudy)
                            .game_icon(game_icon)
                            .coin_info(coin_info)
                            .coin_info270(coin_info270)
                            .spinner_image(spinner_image, spinner_rect)
                            .border_color(color::YELLOW)
                            .border(15.0)
                            .toggle(false);                    
                    item.set(j, ui);
                }
                Event::Selection(selected_id) => {
                    if let &mut Some(_s) = buyselected {
                        if _s == selected_id {
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
    }
}
fn show_result(ui: &mut conrod_core::UiCell,
               ids: &Ids,
               players: &Vec<Player>,
               winner: usize,
               overlay2: &mut bool,
               _appdata: &AppData,
               result_map: &HashMap<ResourceEnum, SupportIdType>) {
    if let Some(_p) = players.get(winner) {
        let mut _str = _p.name.clone();
        _str.push_str(" Wins!");
        widget::Text::new(&_str)
            .color(color::WHITE)
            .font_size(60)
            .h(100.0)
            .top_left_with_margins_on(ids.body, 80.0, 100.0)
            .w_of(ids.body)
            .middle_of(ids.body)
            .set(ids.body_subject_text, ui);
    }


    let item_h = 100.0;
    let (mut items, scrollbar) = widget::List::flow_down(players.len())
        .item_size(item_h)
        .mid_bottom_with_margin_on(ids.body, 20.0)
        .h(item_h * 1.2)
        .padded_w_of(ids.body, 20.0)
        .scrollbar_next_to()
        .set(ids.listview, ui);
    if let Some(s) = scrollbar {
        s.set(ui)
    }
    if let Some(&SupportIdType::ImageId(game_icon)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::GAMEICONS)) {
        let default_color = color::GREY;

        while let Some(item) = items.next(ui) {
            let i = item.i;
            if let Some(_p) = players.get(i) {
                let icon_v = graphics_match::gameicons_listitem(game_icon,
                                                                _p.ink.clone(),
                                                                _p.remover.clone(),
                                                                _p.coin.clone(),
                                                                _p.literacy_award.clone(),
                                                                _p.vp.clone(),
                                                                _p.draftlen.clone());
                let icon_vpliteracyink = icon_v.iter()
                    .enumerate()
                    .filter(|&(_i, _)| (_i == 3) | (_i == 4) | (i == 0))
                    .map(|x| x.1.clone())
                    .collect::<Vec<player_info::item::IconStruct>>();
                let slist = player_info::list::List::new(icon_vpliteracyink, overlay2)
                    .color(default_color)
                    .label(&_p.name)
                    .label_color(default_color.plain_contrast());
                item.set(slist, ui);
            }

        }
    }
}
