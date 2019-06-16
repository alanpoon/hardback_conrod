use conrod_core::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::sprite::{SpriteInfo, spriteable_rect};
use cardgame_widgets::custom_widget::promptview::PromptView;
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids, GuiState};
use graphics_match;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use logic;
use app::PromptSender;
pub fn render(ui: &mut conrod_core::UiCell,
              ids: &Ids,
              gamedata: &mut GameData) {
    if let &Some(_) = &gamedata.overlay_prompt {
        let promptsender = PromptSender();
        let prompt_j = PromptView::new(&mut gamedata.overlay_prompt, promptsender)
            .color(color::LIGHT_GREY)
            .wh_of(ids.master)
            .middle_of(ids.master);
        prompt_j.set(ids.promptview, ui);
        
    }
}
