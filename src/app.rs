use page_curl::page::{self, Page};
use backend::meta::app::Sprite;
use conrod_chat::custom_widget::chatview::Message;
use backend::codec_lib::codec::*;
widget_ids! {
    pub struct Ids {
         master,
         footer,
         footerprevious,
         footernext,
         menubut_multiplayer,
        //lobby
        middle_tabview,
        new_table_but,
        name_text,
        user_name,
        name_rect,
        name_text_edit,
        name_change_but,
        table_list,
        //submitword
        arranged_view,
        hand_view,

         body,
         text,
         prompt_rect,
         prompt_header,
         prompt_logo
    }
}

#[derive(Debug,Clone)]
pub enum GameState {
    Menu,
    Lobby,
    Loading,
    Start,
    Tutorial,
    SubmitWord,
    BuyWord,
}

pub struct GameData {
    pub gamestate: GameState,
    pub footer: Footer,
    pub page_vec: Vec<(Page, Sprite)>,
    pub page_index: usize,
    pub player_size: usize,
    pub lobby_history: Vec<Message>,
    pub lobby_textedit: String,
    pub game_history: Vec<Message>,
    pub game_textedit: String,
    pub name: String,
    pub name_text_edit: String,
    pub players: Vec<Player>,
    pub tables: Vec<TableInfo>,
    pub tablenumber: Option<usize>,
    pub connected: bool,
    pub error_str: Option<String>,
    pub boardcodec: Option<BoardCodec>,
}
impl GameData {
    pub fn new() -> GameData {
        GameData {
            gamestate: GameState::Menu,
            footer: Footer::ShowHand,
            page_vec: vec![(Page::new(), Sprite::PAGE1F),
                           (Page::new(), Sprite::PAGE2F),
                           (Page::new(), Sprite::PAGE3F),
                           (Page::new(), Sprite::PAGE4F)],
            page_index: 0,
            player_size: 4,
            lobby_history: vec![],
            lobby_textedit: "".to_owned(),
            game_history: vec![],
            game_textedit: "".to_owned(),
            name: "".to_owned(),
            name_text_edit: "".to_owned(),
            players: vec![],
            tables: vec![],
            tablenumber: None,
            connected: false,
            error_str: None,
            boardcodec: None,
        }
    }
}
#[derive( Debug, Clone,PartialEq)]
pub enum Footer {
    ShowHand,
    None,
}
