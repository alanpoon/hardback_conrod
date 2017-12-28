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
                                   appdata: &AppData,
                                   result_map: &HashMap<ResourceEnum, SupportIdType>)
                                   -> (bool, &'a str, Color, text::font::Id, Rect) {

    let &meta::cards::BlowupCard { ref theme, 
 //ref crop,ref genre_string,ref non_genre_string,
  .. } =
        appdata.blowupcards.get(&card_index).unwrap();
    let codec_lib::cards::ListCard::<BoardStruct>{
     ref cost,
     ref letter,
     ref genre,
     ref timeless,..
     //ref id,ref cost,ref purchase_giveables,ref giveables,ref genre_giveables,ref trash,ref timeless,..
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
    if let Some(&SupportIdType::FontId(__font)) = result_map.get(&ResourceEnum::Font(font)) {
        _font = Some(__font.clone());
    }
    let source_rect = if *timeless {
        let rect_dim = spriteable_rect(graphics_match::get_cost_info_sprite(), cost.clone() as f64);
        Rect::from_corners(rect_dim.0, rect_dim.1)
    } else {
        let rect_dim = spriteable_rect(graphics_match::get_cost_info_sprite(), cost.clone() as f64);
        Rect::from_corners(rect_dim.0, rect_dim.1)
    };
    (timeless.clone(), letter, color, _font.unwrap(), source_rect)
}
/*
pub fn get_tile_info(card_index:usize,cardmeta: &[codec_lib::cards::ListCard<BoardStruct>; 180],appdata:&AppData){
 let &meta::cards::BlowupCard { ref theme, ref genre_string,ref non_genre_string, .. } =
        appdata.blowupcards.get(&card_index).unwrap();
 let &codec_lib::cards::ListCard::<BoardStruct>{
     ref id,ref letter,ref cost,ref purchase_giveables,ref giveables,ref genre_giveables,ref trash,ref genre,ref timeless,..
 } = cardmeta[card_index.clone()];

       // (theme.clone(),genre.clone(),non_genre.clone())
}
*/
