use page_curl::page::Page;
use backend::meta::app::Sprite;
use backend::meta::cards;
use conrod_chat::custom_widget::chatview::Message;
use backend::codec_lib::codec::*;
use conrod::{image, Rect};
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::time::Instant;
widget_ids! {
    pub struct Ids {
         master,
         footer,
         footerprevious,
         footernext,
         footer_overlay_but,
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
         //notification
         notification_view,
         //in_game
         handview,
         listview,
         shuffleview,
         instructionview,
         //spell
         bodydragdroplistview,
         footerdragdroplistview,
         spell_tab_view,
         //turn to submit
         submit_but,
         //overlay
         overlay,
         overlaytop,
         overlaybody,
         overlaybody_tabview,
         overlay_subject,
         overlay_insufficent_text,
         overlay_explainink,
         overlay_explainlistselect,
         overlay_image_panels,
         overlay_okbut,
         overlay_receivedimage,
         overlay_player_info,
         overlay2_canvas,
         overlay2_image,
         overlay2_text,
         //promptview
         promptview,
         //buy
         body_header_text,
         body_subject_text,
         listselect_view,
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum GuiState {
    Menu,
    Lobby,
    Loading,
    Game(GameState),
}
#[derive(Debug,Clone,PartialEq)]
pub enum OverlayStatus {
    Loading,
    Received(image::Id, Rect, cards::CardType),
    None,
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
    pub initial_draft: Vec<usize>,
    pub keypad_on: bool,
    pub overlay: bool,
    pub overlay_receivedimage: [OverlayStatus; 3],
    pub overlay_index: Option<usize>,
    pub overlay_remover_selected: HashSet<usize, RandomState>,
    pub overlay_timeless_selected: Vec<HashSet<usize, RandomState>>,
    pub overlay2: bool,
    pub buy_selected: Option<usize>,
    pub notification: Option<(String, Instant)>,
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
            initial_draft: vec![],
            keypad_on: false,
            overlay: false,
            overlay_receivedimage: [OverlayStatus::None, OverlayStatus::None, OverlayStatus::None],
            overlay_index: None,
            overlay_remover_selected: HashSet::new(),
            overlay_timeless_selected: vec![HashSet::new(),
                                            HashSet::new(),
                                            HashSet::new(),
                                            HashSet::new()],
            overlay2: false,
            buy_selected: None,
            notification: None,
        }
    }
}
#[derive( Debug, Clone,PartialEq)]
pub enum Footer {
    ShowHand,
    None,
}
