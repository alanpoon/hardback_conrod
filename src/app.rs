use conrod::backend::glium::glium;
use page_curl::page::{Page,self};
use backend::meta::app::Sprite;
widget_ids! {
    pub struct Ids {
         master,
         footer,
         body,
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
    pub page_vec:Vec<(Page,Sprite)>,
}
impl GameData {
    pub fn new() -> GameData {
        GameData {
            gamestate: GameState::Menu,
            footer: Footer::ShowHand,
            page_vec:vec![(Page::new(),Sprite::PAGE1_F),(Page::new(),Sprite::PAGE2_F),(Page::new(),Sprite::PAGE3_F),(Page::new(),Sprite::PAGE4_F)],

        }
    }
   
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub enum Footer {
    ShowHand,
    None,
}
