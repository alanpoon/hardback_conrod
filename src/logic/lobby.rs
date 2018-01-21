use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::{tabview, table_list};
use cardgame_widgets::sprite::{SpriteInfo, spriteable_rect};
use cardgame_widgets::custom_widget::animated_canvas;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite, Font};
use graphics_match;
use logic::in_game;
use logic;
use support;
pub struct TableListTex<'a> {
    appdata: &'a AppData,
}
impl<'a> table_list::TableListTexts for TableListTex<'a> {
    fn text_ready(&self) -> &'static str {
        self.appdata.texts.ready
    }
    fn text_leave(&self) -> &'static str {
        self.appdata.texts.leave
    }
    fn text_join(&self) -> &'static str {
        self.appdata.texts.join
    }
    fn text_playergame(&self) -> &'static str {
        self.appdata.texts.playergame
    }
    fn text_changeto(&self) -> &'static str {
        self.appdata.texts.changeto
    }
}
#[allow(unused_mut)]
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {
    let keypadlength = if gamedata.keypad_on {
        appdata.convert_h(250.0)
    } else {
        0.0
    };
    animated_canvas::Canvas::new()
        .pad_top(appdata.convert_h(40.0))
        .flow_down(&[(ids.overlaybody_chat,
                      animated_canvas::Canvas::new().color(color::LIGHT_BLUE)),
                     (ids.overlaykeypad_chat,
                      animated_canvas::Canvas::new()
                          .color(color::LIGHT_BLUE)
                          .length(keypadlength))])
        .color(color::TRANSPARENT)
        .frame_rate(30)
        .set(ids.master, ui);

    if let Some(mut items) = tabview::TabView::new(vec![appdata.texts.lobby, appdata.texts.chat])
           .padded_wh_of(ids.overlaybody_chat, 0.0)
           .bar_thickness(appdata.convert_h(60.0))
           .middle_of(ids.overlaybody_chat)
           .set(ids.middle_tabview, ui) {
        let vec_closure = render_closure();
       /* let mut it_j = vec_closure.iter();
        while let (Some(a), Some(item)) = (it_j.next(), items.next(ui)) {
            let action_tx_clone = action_tx.clone();
            (*a)(item,
                 ids,
                 gamedata,
                 appdata,
                 result_map,
                 action_tx_clone,
                 ui);
        }
        */
        let mut c=0;
        while let  Some(item) = items.next(ui) {
            let action_tx_clone = action_tx.clone();
            let u= vec_closure.get(c).unwrap();
            (*u)(item,
                 ids,
                 gamedata,
                 appdata,
                 result_map,
                 action_tx_clone,
                 ui);
                 c+=1;
        }
    }


    fn render_closure()
        -> Vec<Box<Fn(tabview::Item,
                      &Ids,
                      &mut GameData,
                      &AppData,
                      &HashMap<ResourceEnum, SupportIdType>,
                      mpsc::Sender<OwnedMessage>,
                      &mut conrod::UiCell)>>
    {
        vec![Box::new(|w_id, ids, mut gamedata, appdata, result_map, action_tx, ui| {
            //draw lobby
            draw_lobby(ui,
                       w_id,
                       ids,
                       &mut gamedata,
                       &appdata,
                       result_map,
                       action_tx);
        }),
             Box::new(|w_id, ids, mut gamedata, _appdata, result_map, action_tx, ui| {
            //Chat
            logic::top_left::draw_lobby_chat(w_id, ids, &mut gamedata, result_map, action_tx, ui);
        })]
    }

    #[allow(unused_mut)]
    fn draw_lobby(ui: &mut conrod::UiCell,
                  w_id: tabview::Item,
                  ids: &Ids,
                  mut gamedata: &mut GameData,
                  appdata: &AppData,
                  result_map: &HashMap<ResourceEnum, SupportIdType>,
                  action_tx: mpsc::Sender<OwnedMessage>) {
        let _table_list_texts = TableListTex { appdata: &appdata };
        if let (Some(&SupportIdType::FontId(bold_font)),
                Some(&SupportIdType::FontId(italic_font))) =
            (result_map.get(&ResourceEnum::Font(Font::BOLD)),
             result_map.get(&ResourceEnum::Font(Font::ITALIC))) {
            let wh = ui.wh_of(ids.middle_tabview).unwrap();
            if let (&app::GuiState::Lobby, None) = (&gamedata.guistate, gamedata.tablenumber) {
                if widget::Button::new()
                       .label(appdata.texts.newtable)
                       .label_color(color::BLACK)
                       .top_left_with_margins_on(w_id.parent_id, 0.0, 0.0)
                       .w_h(wh[0] * 0.3, wh[1] * 0.06)
                       .set(ids.new_table_but, ui)
                       .was_clicked() {
                    let g = json!({
                            "newTable": true
                            });
                    action_tx.clone()
                        .send(OwnedMessage::Text(g.to_string()))
                        .wait()
                        .unwrap();
                };
                let _button_panel = ui.rect_of(ids.new_table_but).unwrap();
                widget::Text::new(appdata.texts.playername)
                    .color(color::WHITE)
                    .font_id(italic_font)
                    .down_from(ids.new_table_but, 2.0)
                    .w_h(appdata.convert_w(200.0), appdata.convert_h(wh[1] * 0.06))
                    .set(ids.name_text, ui);

                widget::Text::new(&gamedata.name)
                    .color(color::WHITE)
                    .font_id(bold_font)
                    .right_from(ids.name_text, 0.0)
                    .w_h(appdata.convert_w(200.0), appdata.convert_h(wh[1] * 0.06))
                    .set(ids.user_name, ui);
                widget::Rectangle::fill_with([appdata.convert_w(200.0), wh[1] * 0.06],
                                             color::WHITE)
                        .right_from(ids.user_name, 0.0)
                        .set(ids.name_rect, ui);
                support::textedit(&mut gamedata.name_text_edit,
                                  ids.name_text_edit,
                                  appdata,
                                  result_map,
                                  [appdata.convert_w(195.0), wh[1] * 0.06],
                                  &mut gamedata.keypad_on,
                                  ids.user_name,
                                  wh[0] * 0.025,
                                  ids.master,
                                  ui);
                if widget::Button::new()
                       .label(appdata.texts.changename)
                       .label_font_size(14)
                       .label_color(color::BLACK)
                       .right_from(ids.name_rect, 2.0)
                       .w_h(wh[0] * 0.3, wh[1] * 0.06)
                       .set(ids.name_change_but, ui)
                       .was_clicked() {
                    gamedata.name = gamedata.name_text_edit.clone();
                    gamedata.name_text_edit = "".to_owned();
                    let g = json!({
                            "namechange": gamedata.name_text_edit.clone()
                            });
                    action_tx.clone()
                        .send(OwnedMessage::Text(g.to_string()))
                        .wait()
                        .unwrap();
                }

            } else {
                widget::Text::new("")
                    .top_left_with_margins_on(w_id.parent_id, 0.0, 0.0)
                    .set(ids.name_text, ui);
            }
            let _name_text_panel = ui.rect_of(ids.name_text).unwrap();
            let item_h = wh[1] * 0.2;
            let (mut items, scrollbar) = widget::List::flow_down(gamedata.tables.len())
                .item_size(item_h)
                .scrollbar_next_to()
                .down_from(ids.name_text, 1.0)
                .padded_wh_of(w_id.parent_id, 4.0)
                .set(ids.table_list, ui);
            if let Some(s) = scrollbar {
                s.set(ui)
            }

            let mut table_iter = gamedata.tables.iter();
            let mut c = 0;
            while let (Some(tableinfo), Some(item)) = (table_iter.next(), items.next(ui)) {
                let c_c = c.clone();
                let j = table_list::TableList::new(&_table_list_texts,
                                                   //ready
                                                   Box::new(|| {
                    let g = json!({
                            "ready":true,
                            });
                    let action_tx_c = action_tx.clone();
                    action_tx_c.send(OwnedMessage::Text(g.to_string())).wait().unwrap();
                }),
                                                   //join
                                                   Box::new(|| {
                    let g = json!({
                            "joinTable":c_c,
                            });
                    let action_tx_c = action_tx.clone();
                    action_tx_c.send(OwnedMessage::Text(g.to_string())).wait().unwrap();
                }),
                                                   //leave
                                                   Box::new(|| {
                    let g = json!({
                            "leavetable":false,
                            });
                    let action_tx_c = action_tx.clone();
                    action_tx_c.send(OwnedMessage::Text(g.to_string())).wait().unwrap();
                }),
                                                   //change_player_number
                                                   Box::new(|x| {
                    let action_tx_c = action_tx.clone();
                    let g = json!({
                            "changePlayers":x,
                            });
                    action_tx_c.send(OwnedMessage::Text(g.to_string())).wait().unwrap();
                }),
                                                   &tableinfo.players,//players
                                                   tableinfo.numberOfPlayers.clone(),//table_space
                                                   4,//max_space
                                                   Some(c) == gamedata.tablenumber//joined
                                                   )
                        .label_color(color::GREEN);
                item.set(j, ui);
                c += 1;
            }

        }
    }
}
