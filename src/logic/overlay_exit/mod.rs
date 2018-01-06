use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable, Rect};
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::sprite::{SpriteInfo, spriteable_rect};
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
use cardgame_widgets::custom_widget::promptview::PromptSendable;
use app::PromptSender;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              action_tx: mpsc::Sender<OwnedMessage>) {
    if gamedata.overlay_exit {
        if let Some(&SupportIdType::ImageId(keypad_image)) =
            result_map.get(&ResourceEnum::Sprite(Sprite::KEYPAD)) {
            let default_color = color::LIGHT_BLUE;
            let close_rect = spriteable_rect(graphics_match::keypad_sprite(), 2.0);
            if animated_canvas::Canvas::new()
                   .middle_of(ids.master)
                   .padded_w_of(ids.master, 100.0)
                   .padded_h_of(ids.master, 200.0)
                   .flow_down(&[(ids.overlaybody_exit,
                                 animated_canvas::Canvas::new().color(default_color))])
                   .color(color::TRANSPARENT)
                   .parent(ids.master)
                   .close_icon_color(color::YELLOW)
                   .close_icon_dim([30.0, 30.0])
                   .close_icon(keypad_image)
                   .close_icon_src_rect(Rect::from_corners(close_rect.0, close_rect.1))
                   .frame_rate(30)
                   .set(ids.overlay_exit, ui)
                   .is_done() {
                gamedata.overlay_exit = false;
            }
            widget::Text::new(&appdata.texts.are_you_sure_exit)
                .font_size(30.0)
                .mid_top_of(ids.overlaybody_exit)
                .color(color::WHITE)
                .set(ids.overlaytext_exit, ui);
            let button_color = color::LIGHT_ORANGE;
            for _i in widget::Button::new()
                    .label(&appdata.texts.yes)
                    .color(button_color)
                    .label_color(button_color.plain_contrast())
                    .wh(appdata.convert_dim([120.0, 100.0]))
                    .bottom_left_with_margin_on(ids.overlaybody_exit, 20.0)
                    .set(ids.overlayyes_exit, ui) {
                let promptsender = PromptSender(action_tx.clone());
                let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
                let mut g = GameCommand::new();
                g.exit_game = Some(true);
                h.set_gamecommand(g);
                promptsender.clone().send(ServerReceivedMsg::serialize_send(h).unwrap());
                gamedata.guistate = GuiState::Menu;
                gamedata.reset();
            }
            for _i in widget::Button::new()
                    .label(&appdata.texts.no)
                    .color(button_color)
                    .label_color(button_color.plain_contrast())
                    .wh(appdata.convert_dim([120.0, 100.0]))
                    .bottom_right_with_margin_on(ids.overlaybody_exit, 20.0)
                    .set(ids.overlayno_exit, ui) {
                gamedata.overlay_exit = false;
            }
        }

    }
}
