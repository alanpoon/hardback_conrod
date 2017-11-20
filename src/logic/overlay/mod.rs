use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::custom_widget::tabview;
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{self, GameData, Ids, Personal};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::{cards, gameicon};
use backend::meta::local;
use logic::{overlay, in_game};
use instruction::Instruction;
pub mod use_ink;
pub fn render(ui: &mut conrod::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &HashMap<ResourceEnum, SupportIdType>,
              _action_tx: mpsc::Sender<OwnedMessage>) {

    if gamedata.overlay.clone {
        animated_canvas::Canvas::new()
            .pad(200.0)
            .flow_down(&[(ids.overlaytop, animated_canvas::Canvas::new().color(color::BLUE)),
                         (ids.overlaybody,
                          animated_canvas::Canvas::new().color(color::BLUE).length(300.0))])
            .color(color::LIGHT_BLUE)
            .watch_state(gamedata.guistate.clone())
            .close_icon(rust_logo)
            .frame_rate(30)
            .set(ids.overlay, ui);

        if let Some(mut items) = tabview::TabView::new(vec![appdata.texts.use_ink,
                                                            appdata.texts.use_remover,
                                                            appdata.texts.use_timelessclassic])
                   .padded_w_of(ids.overlaybody, 0.0)
                   .h(tab_height)
                   .mid_top_of(ids.overlaybody)
                   .set(ids.overlaybody_tabview, ui) {
            let vec_closure = render_closure();
            let mut it_j = vec_closure.iter();
            while let (Some(a), Some(item)) = (it_j.next(), items.next(ui)) {
                let action_tx_clone = action_tx.clone();
                (*a)(item,
                     ids,
                     gamedata,
                     appdata,
                     result_map,
                     action_tx_clone,
                     ui);
            }

        }
    }

}
fn render_closure()
    -> Vec<Box<Fn(tabview::Item,
                  &Ids,
                  &mut GameData,
                  &AppData,
                  &HashMap<ResourceEnum, SupportIdType>,
                  mpsc::Sender<OwnedMessage>,
                  &mut conrod::UiCell)>>
{
    vec![Box::new(|w_id, ids, mut gamedata, appdata, result_map, action_tx, ui| {
        //draw use ink
        overlay::use_ink::render(ui,
                                 w_id,
                                 ids,
                                 &mut gamedata,
                                 &appdata,
                                 result_map,
                                 action_tx);
    }),
         Box::new(|w_id, ids, mut gamedata, _appdata, result_map, action_tx, ui| {
        //draw use remover
        overlay::use_remover::render(w_id, ids, &mut gamedata, result_map, action_tx, ui);
                  }),
         Box::new(|w_id, ids, mut gamedata, _appdata, result_map, action_tx, ui| {
        //draw use timeless classics
        draw_use_timeless_classics(w_id, ids, &mut gamedata, result_map, action_tx, ui);
    })]
}
