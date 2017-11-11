use app::{GameData, GuiState};
use backend::codec_lib::codec::*;
use backend::meta::app::{ResourceEnum, Sprite};
use backend::SupportIdType;
use std::collections::HashMap;
use conrod_chat::chat;
//use animation;
#[allow(unused_variables,non_snake_case)]
pub fn update(s: ClientReceivedMsg,
              gamedata: &mut GameData,
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let ClientReceivedMsg { type_name,
                            tables,
                            tablenumber,
                            request,
                            turn_index,
                            reason,
                            optional,
                            location,
                            privateInformation,
                            sender,
                            message,
                            boardstate,
                            player_index,
                            log,
                            .. } = s;
    if let (Some(Some(_type_name)),
            Some(Some(_location)),
            Some(Some(_sender)),
            Some(Some(_message))) = (type_name.clone(), location, sender, message) {
        if _type_name == "chat" {
            if _location == "lobby" {
                println!("message received {:?}", _message);
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

    if let Some(Some(_player_index)) = player_index {
        gamedata.player_index = Some(_player_index);
    }
    if let (Some(_player_index), Some(Some(Ok(_boardcodec)))) =
        (gamedata.player_index.clone(), boardstate) {
        gamedata.guistate = GuiState::Game(_boardcodec.gamestates
                                               .get(_player_index)
                                               .unwrap()
                                               .clone());
        gamedata.boardcodec = Some(_boardcodec);

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
