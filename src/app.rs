use page_curl::page::Page;
use backend::meta::app::{Texture, Sprite};
use backend::meta::cards;
use conrod_chat::custom_widget::Message;
use backend::codec_lib::codec::*;
use backend::codec_lib::cards::{Board, WaitForInputType};
use conrod::{image, Rect, widget};
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::time::Instant;
use futures::sync::mpsc;
use futures::{Future, Sink};
use cardgame_widgets::custom_widget::promptview::PromptSendable;
use backend::OwnedMessage;
//
widget_ids! {
    pub struct Ids {
         master,
         footer,
         //server_lookup
         server_lookup,
         server_lookup_text_edit,
         footerprevious,
         footernext,
         footer_overlay_but,
         footer_overlay_but2, //chat
         footer_overlay_but3, //exit
         footer_overlay_but4,
         //menu
         menu_title_list1,
         unofficial_logo,
         menu_waiting_connection,
         menubut_multiplayer,
         menu_progessbar,
         menu_version_num,
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
         footer_listselect_view,
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
         corner_arrow,
         //chat
         overlay_chat,
         overlaybody_chat,
         overlaykeypad_chat,
         overlaybody_tabview_chat,
         overlay_exit,
         overlaybody_exit,
         overlaytext_exit,
         overlayyes_exit,
         overlayno_exit,
         overlay_prompt,
         overlayerbody_prompt,
         overlaypromptview_prompt,
         //overlay human
         overlay_human,
         overlay_human_list,
         //loading
         progress_bar,
         loading_gif,
         //blow_up
         blowup_card,
         blowup_list,
         
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum GuiState {
    ServerLookup,
    Menu,
    Lobby,
    Loading,
    Game(GameState),
}
#[derive(Debug,Clone,PartialEq)]
pub enum OverlayStatus {
    Loading,
    Received(usize), //card index
    None,
}
#[derive(Clone)]
pub struct PromptSender(pub mpsc::Sender<OwnedMessage>);
impl PromptSendable for PromptSender {
    fn send(&self, msg: String) {
        self.0
            .clone()
            .send(OwnedMessage::Text(msg))
            .wait()
            .unwrap();
    }
}
pub struct GameData {
    pub version: &'static str,
    pub guistate: GuiState,
    pub footer: Footer,
    pub server_lookup:String,
    pub server_lookup_txt:String,
    pub page_vec: Vec<(Page, Texture)>,
    pub page_index: usize,
    pub go_to_page_index: Option<usize>,
    pub player_size: usize,
    pub lobby_history: Vec<Message>,
    pub lobby_textedit: String,
    pub game_history: Vec<Message>,
    pub game_textedit: String,
    pub name: String,
    pub name_text_edit: String,
    pub tables: Vec<TableInfo>,
    pub tablenumber: Option<usize>,
    pub error_str: Option<String>,
    pub boardcodec: Option<BoardCodec>,
    pub personal: Option<Personal>,
    pub spell_which_arrangelist: Option<widget::Id>,
    pub player_index: Option<usize>,
    pub print_instruction_set: Vec<bool>,
    pub print_instruction_cache: usize,
    pub initial_draft: Vec<usize>,
    pub keypad_on: bool,
    pub overlay: bool,
    pub overlay_chat: bool,
    pub overlay_exit: bool,
    pub overlay_human: bool,
    pub overlay_receivedimage: [OverlayStatus; 3],
    pub overlay_index: Option<usize>,
    pub overlay_remover_selected: HashSet<usize, RandomState>,
    pub overlay_timeless_selected: Vec<HashSet<usize, RandomState>>,
    pub overlay2: bool,
    pub overlay_prompt: Option<(f64, String, Vec<(String, Box<Fn(PromptSender)>)>)>,
    pub overlay_blowup: Option<usize>,
    pub buy_selected: Option<usize>,
    pub notification: Option<(String, Instant)>,
    pub last_send: Option<Instant>, //used in arranging cards before submit
    pub connection_status: ConnectionStatus,
}
impl GameData {
    pub fn new() -> GameData {
        GameData {
            version: "v0.1.0",
            guistate: GuiState::ServerLookup,
            footer: Footer::ShowHand,
            server_lookup:"".to_owned(),
            server_lookup_txt:"".to_owned(),
            page_vec: vec![(Page::new(), Texture::PAGE1F),
                           (Page::new(), Texture::PAGE2F),
                           (Page::new(), Texture::PAGE3F),
                           (Page::new(), Texture::PAGE4F)],
            page_index: 0,
            go_to_page_index: None,
            player_size: 4,
            lobby_history: vec![],
            lobby_textedit: "tasd".to_owned(),
            game_history: vec![],
            game_textedit: "".to_owned(),
            name: "Default Player".to_owned(),
            name_text_edit: "asd".to_owned(),
            tables: vec![],
            tablenumber: None,
            error_str: None,
            boardcodec: None,
            personal: None,
            spell_which_arrangelist: None,
            player_index: None,
            print_instruction_set: vec![true],
            print_instruction_cache: 0,
            initial_draft: vec![],
            keypad_on: false,
            overlay: false,
            overlay_chat: false,
            overlay_exit: false,
            overlay_human: false,
            overlay_receivedimage: [OverlayStatus::None, OverlayStatus::None, OverlayStatus::None],
            overlay_index: None,
            overlay_remover_selected: HashSet::new(),
            overlay_timeless_selected: vec![HashSet::new(),
                                            HashSet::new(),
                                            HashSet::new(),
                                            HashSet::new()],
            overlay2: false,
            overlay_prompt: None,
            overlay_blowup: None,
            buy_selected: None,
            notification: None,
            last_send: None,
            connection_status: ConnectionStatus::None,
        }
    }
    pub fn resize_player_len(&mut self, player_len: usize) {
        self.player_size = player_len;
    }
    pub fn reset(&mut self) {
        self.footer = Footer::ShowHand;
        self.game_history = vec![];
        self.server_lookup = "".to_owned();
        self.server_lookup_txt = "".to_owned();
        self.game_textedit = "".to_owned();
        self.boardcodec = None;
        self.personal = None;
        self.spell_which_arrangelist = None;
        self.player_index = None;
        self.print_instruction_set = vec![true];
        self.print_instruction_cache = 0;
        self.initial_draft = vec![];
        self.keypad_on = false;
        self.overlay = false;
        self.overlay_chat = false;
        self.overlay_exit = false;
        self.overlay_human = false;
        self.overlay_receivedimage =
            [OverlayStatus::None, OverlayStatus::None, OverlayStatus::None];
        self.overlay_index = None;
        self.overlay_remover_selected = HashSet::new();
        self.overlay_timeless_selected =
            vec![HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];
        self.overlay2 = false;
        self.overlay_prompt = None;
        self.overlay_blowup = None;
        self.buy_selected = None;
        self.notification = None;
        self.last_send = None;
    }
}
#[derive( Debug, Clone,PartialEq)]
pub enum Footer {
    ShowHand,
    None,
}
pub struct BoardStruct {}

impl Board for BoardStruct {
    fn two_cent_per_adv(&mut self,
                        _player_id: usize,
                        _card_id: usize,
                        _wait_for_input: &mut [WaitForInputType; 4]) {

    }
    fn minus_other_ink(&mut self,
                       _player_id: usize,
                       _card_id: usize,
                       _wait_for_input: &mut [WaitForInputType; 4]) {

    }
    fn lockup_offer(&mut self,
                    _player_id: usize,
                    _card_id: usize,
                    _wait_for_input: &mut [WaitForInputType; 4]) {


    }

    fn uncover_adjacent(&mut self,
                        _player_id: usize,
                        _card_id: usize,
                        _wait_for_input: &mut [WaitForInputType; 4]) {

    }
    fn double_adjacent(&mut self,
                       _player_id: usize,
                       _card_id: usize,
                       _wait_for_input: &mut [WaitForInputType; 4]) {
    }
    fn trash_other(&mut self,
                   _player_id: usize,
                   _card_id: usize,
                   _wait_for_input: &mut [WaitForInputType; 4]) {

    }
    fn one_vp_per_wild(&mut self,
                       _player_id: usize,
                       _card_id: usize,
                       _wait_for_input: &mut [WaitForInputType; 4]) {

    }
    fn putback_or_discard_three(&mut self,
                                _player_id: usize,
                                _card_id: usize,
                                _wait_for_input: &mut [WaitForInputType; 4]) {
    }
}
