use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image};
use conrod::widget::list_select::Event;
use conrod::widget::list::Right;
use conrod::widget::list::Fixed;
use hardback_meta::app::{AppData, ResourceEnum, Font, Sprite};
use hardback_meta::cards;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use logic::game::SupportIdType;
use app::{self, GameData, Ids};
use backend::Message;
use graphics_match::icons;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<Message>) {
match gamedata.footer{
    app::Footer::ShowHand=>{
         draw_hand(ui, ids, gamedata, appdata, result_map);
    }
}
              }
fn draw_hand(ui: &mut conrod::UiCell,
ids: &Ids,
mut gamedata: &mut GameData,
appdata: &AppData,
result_map: &HashMap<ResourceEnum, SupportIdType>) {

       if let Some(&SupportIdType::ImageId(but_logo)) =result_map.get(&ResourceEnum::Sprite(Sprite::BUTTONS)) {

       }
}