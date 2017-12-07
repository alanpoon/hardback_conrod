use app::{GameData, GuiState, OverlayStatus};
use backend::codec_lib::codec::*;
use backend::meta::app::{ResourceEnum, Sprite, AppData};
use backend::SupportIdType;
use std::collections::HashMap;
use conrod_chat::chat;
use logic::in_game;
use std::time::Instant;
#[allow(unused_variables,non_snake_case)]
pub fn update(s: ClientReceivedMsg,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>) {
    let ClientReceivedMsg { type_name,
                            tables,
                            tablenumber,
                            request,
                            turn_index,
                            location,
                            privateInformation,
                            sender,
                            message,
                            boardstate,
                            player_index,
                            notification,
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
    if let (&Some(Some(ref _request)), Some(ref _overlay_index)) =
        (&request, gamedata.overlay_index) {
        //request
        let card_images = in_game::card_images(result_map);
        let (ref _p_i, ref _c_i, ref _string, ref _vecstring, ref _opt) = *_request;
        let (_image_id, _rect, _theme) = if _overlay_index.clone() <= 1 {
            in_game::get_card_widget_image_portrait(_c_i.clone(), &card_images, appdata)
        } else {
            in_game::get_card_widget_image_flexible(_c_i.clone(), &card_images, appdata)
        };
        gamedata.overlay_receivedimage[_overlay_index.clone()] =
            OverlayStatus::Received(_image_id, _rect, _theme);
    }
    if let (Some(Some(ref _request)), None) = (request, gamedata.overlay_index) {
        //request for the prompts
        let (ref _p_i, ref _c_i, ref _string, ref _vecstring, ref _opt) = *_request;
    }
    if let Some(Some(_string)) = notification {
        gamedata.notification = Some((_string, Instant::now()));
    }
    if let (Some(Some(_type_name)), Some(Some(_tables)), Some(_tablenumber)) =
        (type_name, tables, tablenumber) {
        if _type_name == "lobby" {
            gamedata.tables = _tables;
            gamedata.tablenumber = _tablenumber;
        }
    }


}
