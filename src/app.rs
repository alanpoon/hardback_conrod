use page_curl::page::Page;
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
         body,
         text,
         prompt_rect,
         prompt_header,
         prompt_logo,
         //in_game
         handview,
         listview,
         shuffleview,
         instructionview,
         //spell
         bodydragdroplistview,
         footerdragdroplistview,
         footeruseink_but,
         //turn to submit
         submit_but
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum GuiState {
    Menu,
    Lobby,
    Loading,
    Game(GameState),
}

pub struct GameData {
    pub guistate: GuiState,
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
    pub tables: Vec<TableInfo>,
    pub tablenumber: Option<usize>,
    pub connected: bool,
    pub error_str: Option<String>,
    pub boardcodec: Option<BoardCodec>,
    pub personal: Option<Personal>,
    pub player_index: Option<usize>,
    pub print_instruction_set: Vec<bool>,
    pub keypad_on: bool,
}
impl GameData {
    pub fn new() -> GameData {
        GameData {
            guistate: GuiState::Menu,
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
            tables: vec![],
            tablenumber: None,
            connected: false,
            error_str: None,
            boardcodec: None,
            personal: None,
            player_index: None,
            print_instruction_set: vec![true],
            keypad_on: false,
        }
    }
}
#[derive( Debug, Clone,PartialEq)]
pub enum Footer {
    ShowHand,
    None,
}
#[derive( Debug, Clone)]
pub struct Personal {
    //Can change independently
    pub hand: Vec<usize>,
    pub arranged: Vec<(usize, bool, Option<String>)>,
}
