use conrod_core::{self, color, widget, Colorable, Positionable, Widget, Sizeable, Labelable};
use std::collections::HashMap;
use futures::sync::mpsc;
use app::{GameData, Ids, GuiState};
use ui::RESULTMAPLEN;
use cardgame_widgets::custom_widget::animated_canvas;
use cardgame_widgets::custom_widget::progress_bar;
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum};
use backend::codec_lib::codec::GameState;
use crayon::prelude::*;
use crayon_audio::prelude::*;
use crayon_bytes::prelude::*;
#[allow(unused_mut)]
pub fn render(ui: &mut conrod_core::UiCell,
              ids: &Ids,
              gamedata: &mut GameData,
              appdata: &AppData,
              result_map: &mut HashMap<ResourceEnum, SupportIdType>,
              image_map: &mut conrod_core::image::Map<TextureHandle>) {
    animated_canvas::Canvas::new().color(color::TRANSPARENT).frame_rate(30).set(ids.master, ui);
    let mut loaded =0;
    for (key,value) in result_map.iter_mut(){
        let mut new_value:Option<SupportIdType> =None;
        match value{
            SupportIdType::TextureId(texturehandler)=>{
                let old =loaded.clone();
                impl_probe!{"texture",loaded,texturehandler.clone()}
                if loaded>old{
                    if let ResourceEnum::Sprite(_)=key{
                        new_value = Some(SupportIdType::ImageId(image_map.insert(*texturehandler)));
                    }
                }
            },
            SupportIdType::FontBytes(byteshandler)=>{
                impl_probe!{"font",loaded,*byteshandler}
            },
            SupportIdType::AudioId(audiohandler)=>{
                impl_probe!{"audio",loaded,*audiohandler}
            },
            _=>{
                loaded = loaded+1;
            }
        }
        if let Some(image_id) = new_value{
            *value = image_id;
        }
    }
    progress_bar::ProgressBar::new(loaded, RESULTMAPLEN)
        .middle_of(ids.master)
        .wh(appdata.convert_dim([300.0, 200.0]))
        .label("Loading")
        .set(ids.progress_bar, ui);
    println!("loaded {:?}",loaded);
    if loaded >= RESULTMAPLEN {
        gamedata.guistate = GuiState::Game(GameState::ShowDraft);
    }
}
