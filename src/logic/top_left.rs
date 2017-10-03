use conrod;
use conrod_chat::staticapplication as c_app;
use cardgame_widgets::custom_widget::{list_select, tabview, instructionview};
use conrod_chat::custom_widget::chatview_futures;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Font, Sprite};
use app::{self, GameData, Ids, GameState};
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              mut gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {


}
pub fn draw_lobby_chat(w_id: tabview::Item,
                       ids: &Ids,
                       mut gamedata: &mut GameData,
                       result_map: &HashMap<ResourceEnum, SupportIdType>,
                       action_tx: mpsc::Sender<OwnedMessage>,
                       mut ui: &mut conrod::UiCell) {
    if let Some(&SupportIdType::ImageId(rust_img)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::RUST)) {
        let k = chatview_futures::ChatView::new(&mut gamedata.lobby_history,
                                                &mut gamedata.lobby_textedit,
                                                get_chat_styles(),
                                                Some(rust_img),
                                                &gamedata.name,
                                                action_tx,
                                                Box::new(process));
        w_id.set(k, &mut ui);
    }
}
fn get_chat_styles() -> c_app::Static_Style {
    c_app::Application::default().get_static_styles()
}
fn process(name: &String, text: &String) -> OwnedMessage {
    let g = json!({
    "type":"chat",
  "chat": text,
  "location":"lobby"
});
    OwnedMessage::Text(g.to_string())
}
