use std::collections::HashMap;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::cards;
use conrod;
use conrod::{Rect, image};
pub fn card_images(result_map: &HashMap<ResourceEnum, SupportIdType>)
                   -> [Option<conrod::image::Id>; 27] {
    let mut j = [None; 27];
    if let (Some(&SupportIdType::ImageId(cards1)),
            Some(&SupportIdType::ImageId(cards2)),
            Some(&SupportIdType::ImageId(cards3)),
            Some(&SupportIdType::ImageId(cards4)),
            Some(&SupportIdType::ImageId(cards5)),
            Some(&SupportIdType::ImageId(cards6)),
            Some(&SupportIdType::ImageId(cards7)),
            Some(&SupportIdType::ImageId(cards8)),
            Some(&SupportIdType::ImageId(cards9)),
            Some(&SupportIdType::ImageId(cards10)),
            Some(&SupportIdType::ImageId(cards11)),
            Some(&SupportIdType::ImageId(cards12)),
            Some(&SupportIdType::ImageId(cards13)),
            Some(&SupportIdType::ImageId(cards14)),
            Some(&SupportIdType::ImageId(cards15)),
            Some(&SupportIdType::ImageId(cards16)),
            Some(&SupportIdType::ImageId(cards17)),
            Some(&SupportIdType::ImageId(cards18)),
            Some(&SupportIdType::ImageId(cards19)),
            Some(&SupportIdType::ImageId(cards20)),
            Some(&SupportIdType::ImageId(cards21)),
            Some(&SupportIdType::ImageId(cards22)),
            Some(&SupportIdType::ImageId(cards23)),
            Some(&SupportIdType::ImageId(cards24)),
            Some(&SupportIdType::ImageId(cards25)),
            Some(&SupportIdType::ImageId(cards26)),
            Some(&SupportIdType::ImageId(cards27))) =
        (result_map.get(&ResourceEnum::Sprite(Sprite::CARDS1)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS2)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS3)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS4)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS5)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS6)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS7)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS8)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS9)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS10)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS11)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS12)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS13)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS14)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS15)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS16)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS17)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS18)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS19)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS20)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS21)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS22)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS23)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS24)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS25)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS26)),
         result_map.get(&ResourceEnum::Sprite(Sprite::CARDS27))) {
        j[0] = Some(cards1);
        j[1] = Some(cards2);
        j[2] = Some(cards3);
        j[3] = Some(cards4);
        j[4] = Some(cards5);
        j[5] = Some(cards6);
        j[6] = Some(cards7);
        j[7] = Some(cards8);
        j[8] = Some(cards9);
        j[9] = Some(cards10);
        j[10] = Some(cards11);
        j[11] = Some(cards12);
        j[12] = Some(cards13);
        j[13] = Some(cards14);
        j[14] = Some(cards15);
        j[15] = Some(cards16);
        j[16] = Some(cards17);
        j[17] = Some(cards18);
        j[18] = Some(cards19);
        j[19] = Some(cards20);
        j[20] = Some(cards21);
        j[21] = Some(cards22);
        j[22] = Some(cards23);
        j[23] = Some(cards24);
        j[24] = Some(cards25);
        j[25] = Some(cards26);
        j[26] = Some(cards27);
    }
    j
}
pub fn get_card_widget_image_portrait(card_index: usize,
                                      card_images: &[Option<image::Id>; 27],
                                      appdata: &AppData)
                                      -> (image::Id, Rect) {
    let &cards::BlowupCard { ref theme, ref crop, .. } =
        appdata.blowupcards.get(&card_index).unwrap(); //0:portrait
    let meta_image_index = match theme {
        &cards::CardType::Normal(ref _mi, _) => _mi.clone(),
        &cards::CardType::Rotatable(ref _mi, _, _, _) => _mi.clone(),
    };
    let rect = Rect::from_corners(crop[0].0, crop[0].1);
    (card_images[meta_image_index].clone().unwrap(), rect)
}
pub fn get_card_widget_image_flexible(card_index: usize,
                                      card_images: &[Option<image::Id>; 27],
                                      appdata: &AppData)
                                      -> (image::Id, Rect) {
    let &cards::BlowupCard { ref theme, ref crop, .. } =
        appdata.blowupcards.get(&card_index).unwrap(); //0:portrait

    match (theme, crop.clone()) {
        (&cards::CardType::Normal(ref _mi, _), _crop) => {
            let rect = Rect::from_corners(crop[0].0, crop[0].1);
            (card_images[_mi.clone()].clone().unwrap(), rect)
        }
        (&cards::CardType::Rotatable(_, _, ref _mi, _), _crop) => {
            let rect = Rect::from_corners(crop[1].0, crop[1].1);
            (card_images[_mi.clone()].clone().unwrap(), rect)
        }
    }
}
