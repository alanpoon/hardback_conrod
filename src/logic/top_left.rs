use conrod;
use conrod::{Sizeable, Positionable};
use cardgame_widgets::custom_widget::tabview;
use conrod_chat::custom_widget::chatview_futures;
use std::collections::HashMap;
use futures::sync::mpsc;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use app::{GameData, Ids};
pub fn render(_ui: &mut conrod::UiCell,
              _ids: &Ids,
              mut _gamedata: &mut GameData,
              _appdata: &AppData,
              _result_map: &HashMap<ResourceEnum, SupportIdType>,
              _action_tx: mpsc::Sender<OwnedMessage>) {


}
#[cfg(any(feature="android")]
pub fn draw_lobby_chat(w_id: tabview::Item,
                       ids: &Ids,
                       gamedata: &mut GameData,
                       result_map: &HashMap<ResourceEnum, SupportIdType>,
                       action_tx: mpsc::Sender<OwnedMessage>,
                       mut ui: &mut conrod::UiCell) {
    use conrod_chat::chat::{english, sprite};
    if let (Some(&SupportIdType::ImageId(rust_img)), Some(&SupportIdType::ImageId(key_pad))) =
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

        gamedata.keypad_on = w_id.set(k, &mut ui);
    }
}
#[cfg(feature="default")]
pub fn draw_lobby_chat(w_id: tabview::Item,
                       _ids: &Ids,
                       gamedata: &mut GameData,
                       result_map: &HashMap<ResourceEnum, SupportIdType>,
                       action_tx: mpsc::Sender<OwnedMessage>,
                       mut ui: &mut conrod::UiCell) {
    if let Some(&SupportIdType::ImageId(rust_img)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::RUST)) {
        let k = chatview_futures::ChatView::new(&mut gamedata.lobby_history,
                                                &mut gamedata.lobby_textedit,
                                                Some(rust_img),
                                                &gamedata.name,
                                                action_tx,
                                                Box::new(process));

        w_id.set(k, &mut ui);
    }
}
fn process(_name: &String, text: &String) -> OwnedMessage {
    let g = json!({
    "type":"chat",
  "message": text,
  "location":"lobby"
});
    OwnedMessage::Text(g.to_string())
}