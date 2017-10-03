use app::{self, GameData, GameState};
use backend::server_lib::codec::{ServerReceivedMsg, ClientReceivedMsg};
use backend::meta::app::{AppData, ResourceEnum, Font, Sprite};
use backend::SupportIdType;
use std::collections::HashMap;
use conrod_chat::chat;
//use animation;
pub fn update(s: ClientReceivedMsg,
              mut gamedata: &mut GameData,
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let ClientReceivedMsg { type_name,
                            tables,
                            tablenumber,
                            players,
                            request,
                            boardstate,
                            reason,
                            optional,
                            location,
                            privateInformation,
                            sender,
                            message,
                            log } = s;
    if let (Some(Some(_type_name)),
            Some(Some(_location)),
            Some(Some(_sender)),
            Some(Some(_message))) = (type_name.clone(), location, sender, message) {
        if _type_name == "chat" {
            if _location == "lobby" {
                if let Some(&SupportIdType::ImageId(rust_logo)) =
                    result_map.get(&ResourceEnum::Sprite(Sprite::RUST)) {
                    gamedata.lobby_history.push(chat::message::Message {
                                                    image_id: Some(rust_logo),
                                                    name: _sender,
                                                    text: _message,
                                                });
                }

            }
            if _location == "table" {}
        }
    }
    if let (Some(Some(_players)), Some(Some(_log)), Some(Some(_privateInformation))) =
        (players, log, privateInformation) {
        gamedata.gamestate = app::GameState::Start;
        gamedata.players = _players;
        //  gamedata.log = _log;
    }
    if let (Some(Some(_request)), Some(Some(_reason)), Some(Some(_optional))) =
        (request, reason, optional) {
        //prompt
    }
    if let (Some(Some(_type_name)), Some(Some(_tables)), Some(_tablenumber)) =
        (type_name, tables, tablenumber) {
        if _type_name == "lobby" {
            gamedata.tables = _tables;
            gamedata.tablenumber = _tablenumber;
        }
    }
}
