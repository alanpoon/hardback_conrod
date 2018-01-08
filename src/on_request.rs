use app::{GameData, GuiState, OverlayStatus};
use backend::codec_lib::codec::*;
use backend::meta::app::{ResourceEnum, Sprite, AppData};
use backend::SupportIdType;
use std::collections::HashMap;
use conrod_chat::chat;
use logic::in_game;
use std::time::Instant;
use cardgame_widgets::custom_widget::promptview::PromptSendable;
use app::PromptSender;
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
                            hand,
                            connection_status,
                            .. } = s;
    if let Some(Some(_status)) = connection_status.clone() {
        gamedata.connection_status = _status.clone();
        match _status {
            ConnectionStatus::Ok => {}
            ConnectionStatus::Error(ConnectionError::NotConnectedToInternet) => {
                gamedata.notification = Some(("NotConnectedToInternet".to_owned(), Instant::now()));
            }
            ConnectionStatus::Error(ConnectionError::CannotFindServer) => {
                gamedata.notification = Some(("Cannot Find Server".to_owned(), Instant::now()));
            }
            ConnectionStatus::Error(ConnectionError::InvalidDestination) => {
                gamedata.notification = Some(("Invalid Destination".to_owned(), Instant::now()));
            }
            _ => {}
        }
    }
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
            } else if _location == "game" {
                if let Some(&SupportIdType::ImageId(rust_logo)) =
                    result_map.get(&ResourceEnum::Sprite(Sprite::RUST)) {
                    gamedata.game_history.push(chat::message::Message {
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
        let player_len = _boardcodec.players.len();
        gamedata.boardcodec = Some(_boardcodec);

        if gamedata.player_size != player_len {
            gamedata.player_size = player_len;
        }

    }
    if let (&Some(Some(ref _request)), Some(ref _overlay_index)) =
        (&request, gamedata.overlay_index) {
        //request
        let (ref _p_i, ref _c_i, ref _string, ref _vecstring, ref _opt) = *_request;
        gamedata.overlay_receivedimage[_overlay_index.clone()] =
            OverlayStatus::Received(_c_i.clone());
    }
    if let Some(Some(ref _request)) = request {
        //request for the prompts
        let GameData { ref mut overlay_prompt, .. } = *gamedata;
        let (ref _p_i, ref _c_i, ref _string, ref _vecstring, ref _opt) = *_request;
        let vec_closure: Vec<(String, Box<Fn(PromptSender)>)> =
            _vecstring.iter()
                .enumerate()
                .map(|(_i, _x)| {
                    let o: (String, Box<Fn(PromptSender)>) = (_x.clone(),
                                                              Box::new(move |ps| {
                        let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
                        let mut g = GameCommand::new();
                        g.reply = Some(_i.clone());
                        h.set_gamecommand(g);
                        ps.send(ServerReceivedMsg::serialize_send(h).unwrap());
                    }));
                    o
                })
                .collect::<Vec<(String, Box<Fn(PromptSender)>)>>();
        *overlay_prompt = Some((0.5, _string.clone(), vec_closure))

    }
    if let Some(Some(_string)) = notification {
        gamedata.notification = Some((_string, Instant::now()));
    }
    if let Some(Some(_hand)) = hand {
        let GameData { ref mut personal, .. } = *gamedata;
        recache_personal(_hand, personal);

    }
    if let (Some(Some(_type_name)), Some(Some(_tables)), Some(_tablenumber)) =
        (type_name, tables, tablenumber) {
        if _type_name == "lobby" {
            gamedata.tables = _tables;
            gamedata.tablenumber = _tablenumber;
        }
    }



}
fn recache_personal(hand: Vec<usize>, personal: &mut Option<Personal>) {
    *personal = Some(Personal {
                         hand: hand,
                         arranged: vec![],
                     });
}
