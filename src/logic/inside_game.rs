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
use backend::OwnedMessage;
use backend::SupportIdType;
use graphics_match::button;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {
    match gamedata.gamestate {
        app::GameState::InsideGame => {
            draw_hand(ui, ids, gamedata, appdata, result_map);
        }
        _ => {}
    }
}
fn draw_card(ui: &mut conrod::UiCell,
             ids: &Ids,
             mut gamedata: &mut GameData,
             appdata: &AppData,
             result_map: &HashMap<ResourceEnum, SupportIdType>) {
        if let Some(&SupportIdType::ImageId(but_logo)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::BUTTON)) {

        }
             }