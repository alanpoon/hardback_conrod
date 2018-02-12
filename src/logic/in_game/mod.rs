use std::collections::HashMap;
use cardgame_widgets::sprite::{spriteable_rect, SpriteInfo};
use backend::SupportIdType;
use backend::meta;
use backend::meta::app::{AppData, ResourceEnum, Sprite, Font};
use backend::codec_lib::{self, cards};
use backend::codec_lib::cards::*;
use conrod;
use conrod::{Rect, image, color, Color, text};
use app::BoardStruct;
use graphics_match;

pub fn get_tile_image_withcost<'a>(card_index: usize,
                                   cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
                                   _appdata: &AppData,
                                   result_map: &HashMap<ResourceEnum, SupportIdType>)
                                   -> (bool, &'a str, Color, text::font::Id, Rect, Rect) {

    let codec_lib::cards::ListCard::<BoardStruct>{
     ref cost,
     ref letter,
     ref genre,
     ref timeless,ref giveables,..
     //ref id,ref cost,ref purchase_giveables,ref genre_giveables,ref trash,ref timeless,..
 } = cardmeta[card_index.clone()];
    let color = match genre {
        &Genre::MYSTERY => color::LIGHT_BLUE,
        &Genre::HORROR => color::LIGHT_GREEN,
        &Genre::ADVENTURE => color::LIGHT_YELLOW,
        &Genre::ROMANCE => color::LIGHT_RED,
        _ => color::LIGHT_GRAY,
    };
    let font = match genre {
        &Genre::MYSTERY => Font::MYSTERY,
        &Genre::HORROR => Font::HORROR,
        &Genre::ADVENTURE => Font::ADVENTURE,
        &Genre::ROMANCE => Font::ROMANCE,
        _ => Font::BOLD,
    };

    let top_lefticon = match giveables {
        &GIVEABLE::VP(_) => TopLeftIcon::VP,
        &GIVEABLE::COIN(_) => TopLeftIcon::COIN,
        &GIVEABLE::VPCOIN(_, _) => TopLeftIcon::COINVP,
        &GIVEABLE::COININK(_) => TopLeftIcon::COIN,
        &GIVEABLE::VPINK(_) => TopLeftIcon::COINVP,
        &GIVEABLE::VPORCOININK(_) => TopLeftIcon::COINVP,
        _ => TopLeftIcon::COINVP,
    };
    let top_lefticon_src_rect = match top_lefticon {
        TopLeftIcon::VP => {
            let rect_dim = spriteable_rect(graphics_match::gameicon_sprite(), 5.0);
            Rect::from_corners(rect_dim.0, rect_dim.1)
        }
        TopLeftIcon::COIN => {
            let rect_dim = spriteable_rect(graphics_match::gameicon_sprite(), 3.0);
            Rect::from_corners(rect_dim.0, rect_dim.1)
        }
        TopLeftIcon::COINVP => {
            let rect_dim = spriteable_rect(graphics_match::gameicon_sprite(), 3.0);
            Rect::from_corners(rect_dim.0, rect_dim.1)
        }
    };
    let mut _font = None;
    if let Some(&SupportIdType::FontId(__font)) = result_map.get(&ResourceEnum::Font(font)) {
        _font = Some(__font.clone());
    }
    let source_rect = if *timeless {
        let rect_dim = spriteable_rect(graphics_match::get_cost_info270_sprite(),
                                       9.0 - (cost.clone() as f64));
        Rect::from_corners(rect_dim.0, rect_dim.1)
    } else {
        let rect_dim = spriteable_rect(graphics_match::get_cost_info_sprite(),
                                       cost.clone() as f64);
        Rect::from_corners(rect_dim.0, rect_dim.1)
    };

    (timeless.clone(), letter, color, _font.unwrap(), source_rect, top_lefticon_src_rect)
}
pub fn get_tile_image_withcostwords<'a>(card_index: usize,
                                        cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],
                                        appdata: &AppData,
                                        result_map: &HashMap<ResourceEnum, SupportIdType>)
                                        -> (bool,
                                            &'a str,
                                            Color,
                                            text::font::Id,
                                            Rect,
                                            Rect,
                                            GIVEABLE,
                                            GIVEABLE,
                                            GIVEABLE,
                                            GIVEABLE,
                                            Option<String>,
                                            Option<String>) {

    let &meta::cards::BlowupCard { ref genre_string, ref non_genre_string, .. } =
        appdata.blowupcards.get(&card_index).unwrap();
    let codec_lib::cards::ListCard::<BoardStruct>{
     ref cost,
     ref letter,
     ref purchase_giveables,
     ref giveables,
     ref genre_giveables,
     ref trash,
     ref genre,
     ref timeless,..
     //ref id,ref cost,
 } = cardmeta[card_index.clone()];
    let color = match genre {
        &Genre::MYSTERY => color::LIGHT_BLUE,
        &Genre::HORROR => color::LIGHT_GREEN,
        &Genre::ADVENTURE => color::LIGHT_YELLOW,
        &Genre::ROMANCE => color::LIGHT_RED,
        _ => color::LIGHT_GRAY,
    };
    let font = match genre {
        &Genre::MYSTERY => Font::MYSTERY,
        &Genre::HORROR => Font::HORROR,
        &Genre::ADVENTURE => Font::ADVENTURE,
        &Genre::ROMANCE => Font::ROMANCE,
        _ => Font::BOLD,
    };
    let mut _font = None;
    let top_lefticon = match giveables {
        &GIVEABLE::VP(_) => TopLeftIcon::VP,
        &GIVEABLE::COIN(_) => TopLeftIcon::COIN,
        &GIVEABLE::VPCOIN(_, _) => TopLeftIcon::COINVP,
        &GIVEABLE::COININK(_) => TopLeftIcon::COIN,
        &GIVEABLE::VPINK(_) => TopLeftIcon::COINVP,
        &GIVEABLE::VPORCOININK(_) => TopLeftIcon::COINVP,
        _ => TopLeftIcon::COINVP,
    };
    let top_lefticon_src_rect = match top_lefticon {
        TopLeftIcon::VP => {
            let rect_dim = spriteable_rect(graphics_match::gameicon_sprite(), 5.0);
            Rect::from_corners(rect_dim.0, rect_dim.1)
        }
        TopLeftIcon::COIN => {
            let rect_dim = spriteable_rect(graphics_match::gameicon_sprite(), 3.0);
            Rect::from_corners(rect_dim.0, rect_dim.1)
        }
        TopLeftIcon::COINVP => {
            let rect_dim = spriteable_rect(graphics_match::gameicon_sprite(), 3.0);
            Rect::from_corners(rect_dim.0, rect_dim.1)
        }
    };
    if let Some(&SupportIdType::FontId(__font)) = result_map.get(&ResourceEnum::Font(font)) {
        _font = Some(__font.clone());
    }
    let source_rect = if *timeless {
        let rect_dim = spriteable_rect(graphics_match::get_cost_info270_sprite(),
                                       cost.clone() as f64);
        Rect::from_corners(rect_dim.0, rect_dim.1)
    } else {
        let rect_dim = spriteable_rect(graphics_match::get_cost_info_sprite(), cost.clone() as f64);
        Rect::from_corners(rect_dim.0, rect_dim.1)
    };
    (timeless.clone(),
     letter,
     color,
     _font.unwrap(),
     source_rect,
     top_lefticon_src_rect,
     purchase_giveables.clone(),
     giveables.clone(),
     genre_giveables.clone(),
     trash.clone(),
     genre_string.clone(),
     non_genre_string.clone())
}
enum TopLeftIcon {
    COIN,
    VP,
    COINVP,
}
