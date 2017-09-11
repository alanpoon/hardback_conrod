use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use conrod::widget::list_select::Event;
use conrod::widget::list::Right;
use conrod::widget::list::Fixed;
use hardback_meta::app::{AppData, ResourceEnum, Font, Sprite};
use hardback_meta::cards;
use cardgame_widgets::custom_widget::animated_button;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids};
use backend::Message;
use backend::SupportIdType;
use graphics_match::button;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: Option<mpsc::Sender<Message>>) {
    match gamedata.footer {
        app::Footer::ShowHand => {
            draw_hand(ui, ids, gamedata, appdata, result_map);
        }
        _ => {}
    }
}
fn draw_hand(ui: &mut conrod::UiCell,
             ids: &Ids,
             mut gamedata: &mut GameData,
             appdata: &AppData,
             result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let wh = ui.wh_of(ids.footer).unwrap();
    let z = button::get_style();
    if let Some(&SupportIdType::ImageId(but_logo)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::BUTTON)) {
        if animated_button::AnimatedButton::image(but_logo)
               .label(appdata.texts.previous)
               .normal_rect(z.src_rect(19.0))
               .hover_rect(z.src_rect(20.0))
               .press_rect(z.src_rect(20.0))
               .top_left_of(ids.footer)
               .w(0.2 * wh[0])
               .h(0.3 * wh[1])
               .set(ids.footerprevious, ui)
               .was_clicked() {
                   println!("aaaa");
            page_previous(gamedata);
        }
        if animated_button::AnimatedButton::image(but_logo)
               .label(appdata.texts.next)
               .normal_rect(z.src_rect(19.0))
               .hover_rect(z.src_rect(20.0))
               .press_rect(z.src_rect(20.0))
               .top_right_of(ids.footer)
                .w(0.2 * wh[0])
               .h(0.3 * wh[1])
               .set(ids.footernext, ui)
               .was_clicked() {
            page_next(gamedata);
        };
    }
}
fn page_next(gamedata: &mut GameData) {
    if gamedata.page_index + 1 >= gamedata.player_size {
        gamedata.page_index = 0;
        for i in (0usize..gamedata.page_vec.len()).rev(){
            if let Some( &mut(ref mut _page, _)) = gamedata.page_vec.get_mut(i){
                      if i > 0 {
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
fn page_previous(gamedata: &mut GameData) {
    if gamedata.page_index as f32 - 1.0 < 0.0 {
        gamedata.page_index = gamedata.player_size - 1;
          for i in 0..gamedata.page_vec.len(){
                   if let Some( &mut(ref mut _page, _)) = gamedata.page_vec.get_mut(i){
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
