use conrod;
use cardgame_widgets::custom_widget::{list_select, tabview, instructionview};
use conrod_chat::custom_widget::chatview_futures;
use conrod_chat::chat::{english,sprite};
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
    if let (Some(&SupportIdType::ImageId(rust_img)),Some(&SupportIdType::ImageId(key_pad))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::RUST)),
        result_map.get(&ResourceEnum::Sprite(Sprite::KEYPAD))) {
        let english_tuple = english::populate(key_pad, sprite::get_spriteinfo());
        let k = chatview_futures::ChatView::new(&mut gamedata.lobby_history,
                                                &mut gamedata.lobby_textedit,
                                                ids.master,
                                                &english_tuple,
                                                Some(rust_img),
                                                &gamedata.name,
                                                action_tx,
                                                Box::new(process));
        w_id.set(k, &mut ui);
    }
}

fn process(name: &String, text: &String) -> OwnedMessage {
    let g = json!({
    "type":"chat",
  "chat": text,
  "location":"lobby"
});
    OwnedMessage::Text(g.to_string())
}
