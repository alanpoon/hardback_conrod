use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use conrod::widget::list_select::Event;
use conrod::widget::list::Right;
use conrod::widget::list::Fixed;
use cardgame_widgets::custom_widget::{tabview, animated_button, table_list};
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Font, Sprite};
use graphics_match::button;
use logic;
pub struct TableListTex<'a>{
    appdata:&'a AppData,
}
impl<'a> table_list::TableListTexts for TableListTex<'a>{
    fn text_ready(&self)->&'static str{
        self.appdata.texts.ready
    }
    fn text_leave(&self)->&'static str{
        self.appdata.texts.leave
    }
    fn text_join(&self)->&'static str{
        self.appdata.texts.join
    }
    fn text_playergame(&self)->&'static str{
        self.appdata.texts.playergame
    }
    fn text_changeto(&self)->&'static str{
        self.appdata.texts.changeto
    }
}
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {
    widget::Canvas::new().pad(90.0).color(color::GREEN).set(ids.master, ui);
    if let Some(mut items) = tabview::TabView::new(vec![appdata.texts.lobby, appdata.texts.chat])
           .padded_w_of(ids.master, 0.1 * ui.win_h)
           .padded_h_of(ids.master, 0.2 * ui.win_w)
           .middle_of(ids.master)
           .set(ids.middle_tabview, ui) {
        let vec_closure = render_closure();
        let mut it_j = vec_closure.iter();
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
             Box::new(|w_id, ids, mut gamedata, appdata, result_map, action_tx, ui| {
            //Chat
            logic::top_left::draw_lobby_chat(w_id, ids, &mut gamedata, result_map, action_tx, ui);
        })]
    }
    fn draw_lobby(ui: &mut conrod::UiCell,
                  w_id: tabview::Item,
                  ids: &Ids,
                  mut gamedata: &mut GameData,
                  appdata: &AppData,
                  result_map: &HashMap<ResourceEnum, SupportIdType>,
                  action_tx: mpsc::Sender<OwnedMessage>) {
        let _style = button::get_style();
        let _table_list_texts=TableListTex{appdata:&appdata};
        if let Some(&SupportIdType::ImageId(rust_logo)) =
            result_map.get(&ResourceEnum::Sprite(Sprite::BUTTON)) {
            let card_index = 7.0;
            let wh = ui.wh_of(ids.middle_tabview).unwrap();
            // let wh = [200.0,200.0];
            if let app::GameState::Lobby(true) = gamedata.gamestate {
                if animated_button::AnimatedButton::image(rust_logo)
                       .label(appdata.texts.newtable)
                       .label_font_size(14)
                       .label_color(color::LIGHT_BLUE)
                       .normal_rect(_style.src_rect(card_index))
                       .hover_rect(_style.src_rect(card_index + 1.0))
                       .press_rect(_style.src_rect(card_index + 2.0))
                       .top_left_with_margins_on(w_id.parent_id, 0.0, 0.0)
                       .w_h(wh[0] * 0.3, wh[1] * 0.06)
                       .set(ids.new_table_but, ui)
                       .was_clicked() {
                    action_tx.clone()
                        .send(OwnedMessage::Text("{'newTable':true}".to_owned()))
                        .wait()
                        .unwrap();
                };
                let button_panel = ui.rect_of(ids.new_table_but).unwrap();
                widget::Text::new(appdata.texts.playername)
                    .down_from(ids.new_table_but, 2.0)
                    .w_h(wh[0] * 0.3, wh[1] * 0.04)
                    .set(ids.name_text, ui);
                widget::Rectangle::fill_with([wh[0] * 0.3, wh[1] * 0.04], color::WHITE)
                    .right_from(ids.name_text, 0.0)
                    .set(ids.name_rect, ui);
                let mut k = &mut gamedata.name_text_edit;
                for edit in widget::TextEdit::new(k)
            .color(color::BLACK)
            .w_h(wh[0]*0.25, wh[1]* 0.04)
            .right_from(ids.name_text,wh[0]*0.025)
            .left_justify()
            .line_spacing(2.5)
            .restrict_to_height(true) // Let the height grow infinitely and scroll.
            .set(ids.name_text_edit, ui) {
                    *k = edit;
                }

                let change_name_index = 9.0;
                if animated_button::AnimatedButton::image(rust_logo)
                       .label(appdata.texts.changename)
                       .label_font_size(14)
                       .label_color(color::LIGHT_BLUE)
                       .normal_rect(_style.src_rect(change_name_index))
                       .hover_rect(_style.src_rect(change_name_index + 1.0))
                       .press_rect(_style.src_rect(change_name_index + 2.0))
                       .right_from(ids.name_rect, 2.0)
                       .w_h(wh[0] * 0.3, wh[1] * 0.06)
                       .set(ids.name_change_but, ui)
                       .was_clicked() {
                    let f = format!("{{'namechange':'{}'}}", k);
                    action_tx.clone()
                        .send(OwnedMessage::Text(f))
                        .wait()
                        .unwrap();
                }

            } else {
                widget::Rectangle::fill_with([0.0, 0.0], color::WHITE)
                    .top_left_with_margins_on(w_id.parent_id, 0.0, 0.0)
                    .set(ids.name_text, ui);
            }
            let name_text_panel = ui.rect_of(ids.name_text).unwrap();
            let item_h = wh[1] * 0.2;
            let (mut items, scrollbar) = widget::List::flow_down(gamedata.tables.len())
           // .item_size(wh[0])
           .item_size(item_h)
            .scrollbar_next_to()
            .down_from(ids.name_text, 1.0)
            .padded_wh_of(w_id.parent_id,4.0)
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
                                                                let action_tx_c = action_tx.clone();
                                                                action_tx_c.send(OwnedMessage::Text("{'ready':true}".to_owned()))
                    .wait()                                                                                 
                    .unwrap();
                                                            }),
                                                   //join
                                                   Box::new(|| {
                    let f = format!("{{'joinTable':{}}}", c_c);
                    let action_tx_c = action_tx.clone();
                    action_tx_c.send(OwnedMessage::Text(f)).wait().unwrap();
                }),
                                                   //leave
                                                   Box::new(|| {
                                                                let action_tx_c = action_tx.clone();
                                                                action_tx_c.send(OwnedMessage::Text("{'leavetable':false}".to_owned())).wait().unwrap();
                                                            }),
                                                   //change_player_number
                                                   Box::new(|x| {
                    let action_tx_c = action_tx.clone();
                    let f = format!("{{'changePlayers':{}}}", x);
                    action_tx_c.send(OwnedMessage::Text(f)).wait().unwrap();
                }),
                                                   &tableinfo.players,
                                                   tableinfo.numberOfPlayers.clone(),
                                                   Some(c) == gamedata.tablenumber);

                item.set(j, ui);

                c += 1;
            }

        }
    }
}
