use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use conrod::widget::list_select::Event;
use conrod::widget::list::Right;
use conrod::widget::list::Fixed;
use cardgame_widgets::custom_widget::animated_button;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids, GameState};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Font, Sprite};
use graphics_match::button;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>){
                  if !gamedata.connected{
                      render_connecting(ui,ids,gamedata,appdata,result_map);
                  }
              }
pub fn render_connecting(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    widget::Canvas::new().color(color::TRANSPARENT).set(ids.master, ui);
  let Some([w,h]) = ui.wh_of(ids.master){
      widget::Rectangle::fill([w, h])
            .middle()
            .color(color::Pink)
            .set(ids.prompt_rect, ui);
        widget::Text::new("asd").middle_of(ids.prompt_rect,ui);
  }
 
}
