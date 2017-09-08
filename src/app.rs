
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
#[derive(Debug,Clone)]
pub struct GameData {
    pub gamestate: GameState,
    pub footer: Footer,
}
impl GameData {
    pub fn new() -> GameData {
        GameData {
            gamestate: GameState::Menu,
            footer: Footer::ShowHand,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub enum Footer {
    ShowHand,
    None,
}
