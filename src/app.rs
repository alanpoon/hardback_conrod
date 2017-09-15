use conrod::backend::glium::glium;
use page_curl::page::{self, Page};
use backend::meta::app::Sprite;
use conrod_chat::custom_widget::chatview::Message;
use backend::server_lib::codec::*;
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
        name_rect,
        name_text_edit,
        name_change_but,
        table_list,

         body,
         text
    }
}

#[derive(Debug,Clone)]
pub enum GameState {
    Menu,
    Lobby(bool), //true ->new Table
    Loading,
    Start,
    Tutorial,
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
    pub tablenumber: Option<i32>,
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
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub enum Footer {
    ShowHand,
    None,
}
