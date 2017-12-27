use cardgame_widgets::sprite::{SpriteInfo, spriteable_rect};
use conrod::{Rect, image};
use conrod::widget::primitive::image::Image;
use cardgame_widgets::custom_widget::image_hover::Hoverable;
use cardgame_widgets::custom_widget::player_info::item::IconStruct;
use conrod::widget::envelope_editor::EnvelopePoint;

pub struct ImageHoverable(pub Image, pub Option<Image>, pub Option<Image>);
impl Hoverable for ImageHoverable {
    fn idle(&self) -> Image {
        self.0
    }
    fn hover(&self) -> Option<Image> {
        self.1
    }
    fn press(&self) -> Option<Image> {
        self.2
    }
}

pub fn keypad_sprite() -> SpriteInfo {
    SpriteInfo {
        first: (0.0, 400.0),
        num_in_row: 3,
        num_in_col: 2,
        w_h: (200.0, 200.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
}
pub fn spinner_sprite() -> SpriteInfo {
    SpriteInfo {
        first: (0.0, 400.0),
        num_in_row: 12,
        num_in_col: 4,
        w_h: (100.0, 100.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
}
pub fn gameicon_sprite() -> SpriteInfo {
    SpriteInfo {
        first: (0.0, 1600.0),
        num_in_row: 3,
        num_in_col: 4,
        w_h: (400.0, 400.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
}
pub fn gameicons_rect(i: f64) -> Rect {
    let icon_rect = spriteable_rect(gameicon_sprite(), i);
    Rect::from_corners(icon_rect.0, icon_rect.1)
}
pub fn gameicons_listitem(image_id: image::Id,
                          i: usize,
                          r: usize,
                          c: usize,
                          la: usize,
                          p: usize,
                          d: usize)
                          -> Vec<IconStruct> {
    vec![IconStruct(Image::new(image_id).source_rectangle(gameicons_rect(1.0)),i.to_string(),"Ink, like Blackjack, draw one more card in hope of scoring more points. You must however use this card to spell word".to_owned()), //ink
         IconStruct(Image::new(image_id).source_rectangle(gameicons_rect(2.0)),r.to_string(),"Ink Remover, You may convert an inked card back to normal. You put it back into your hand".to_owned()), //inkremover
         IconStruct(Image::new(image_id).source_rectangle(gameicons_rect(3.0)),c.to_string(),"Coin, You may use coin to buy new cards".to_owned()), //coin
         IconStruct(Image::new(image_id).source_rectangle(gameicons_rect(4.0)),la.to_string(),"Literacy Award, Construct longer words, the token go to the last player who constructed the longest word".to_owned()), //literacy award
         IconStruct(Image::new(image_id).source_rectangle(gameicons_rect(5.0)),p.to_string(),"Prestige, End Game Victory Point".to_owned()), //prestige
         IconStruct(Image::new(image_id).source_rectangle(gameicons_rect(6.0)),d.to_string(),"Size of Draft pile".to_owned()), //draftlen
    ]
}
pub fn backcard() -> Rect {
    Rect::from_corners([670.0, 70.0], [1130.0, 850.0])
}
pub fn arrow_sprite() -> SpriteInfo {
    SpriteInfo {
        first: (0.0, 400.0), //left corner of first
        num_in_row: 4,
        num_in_col: 3,
        w_h: (200.0, 200.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
}
pub fn all_arrows
    (button: image::Id)
     -> (ImageHoverable, ImageHoverable, ImageHoverable, ImageHoverable, ImageHoverable) {
    let b_s = arrow_sprite();
    let l_0 = spriteable_rect(b_s, 4.0);
    let l_1 = spriteable_rect(b_s, 6.0);
    let t_0 = spriteable_rect(b_s, 0.0);
    let t_1 = spriteable_rect(b_s, 2.0);
    let r_0 = spriteable_rect(b_s, 1.0);
    let r_1 = spriteable_rect(b_s, 3.0);
    let b_0 = spriteable_rect(b_s, 5.0);
    let b_1 = spriteable_rect(b_s, 7.0);
    let c_0 = spriteable_rect(b_s, 8.0);
    let c_1 = spriteable_rect(b_s, 10.0);
    let left_arrow_z =
        ImageHoverable(Image::new(button).source_rectangle(Rect::from_corners(l_0.0, l_0.1)),
                       Some(Image::new(button).source_rectangle(Rect::from_corners(l_1.0, l_1.1))),
                       None);
    let top_arrow_z =
        ImageHoverable(Image::new(button).source_rectangle(Rect::from_corners(t_0.0, t_0.1)),
                       Some(Image::new(button).source_rectangle(Rect::from_corners(t_1.0, t_1.1))),
                       None);
    let right_arrow_z =
        ImageHoverable(Image::new(button).source_rectangle(Rect::from_corners(r_0.0, r_0.1)),
                       Some(Image::new(button).source_rectangle(Rect::from_corners(r_1.0, r_1.1))),
                       None);
    let btm_arrow_z =
        ImageHoverable(Image::new(button).source_rectangle(Rect::from_corners(b_0.0, b_0.1)),
                       Some(Image::new(button).source_rectangle(Rect::from_corners(b_1.0, b_1.1))),
                       None);
    let corner_arrow_z =
        ImageHoverable(Image::new(button).source_rectangle(Rect::from_corners(c_0.0, c_0.1)),
                       Some(Image::new(button).source_rectangle(Rect::from_corners(c_1.0, c_1.1))),
                       None);
    (left_arrow_z, top_arrow_z, right_arrow_z, btm_arrow_z, corner_arrow_z)
}
pub fn cards_btm(recz: Rect) -> Rect {
    let topleft_c = recz.top_left();
    let mut topleft_cm = topleft_c.clone();
    topleft_cm.set_x(topleft_c.get_x() + 136.0);
    topleft_cm.set_y(topleft_c.get_y() - 400.0);
    Rect::from_corners(topleft_cm, recz.bottom_right())
}
pub fn get_cost_info_sprite() -> SpriteInfo {
    SpriteInfo {
        first: (0.0, 400.0), //left corner of first
        num_in_row: 10,
        num_in_col: 1,
        w_h: (41.0, 400.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
}
pub fn get_cost_info270_sprite() -> SpriteInfo {
    SpriteInfo {
        first: (0.0, 400.0), //left corner of first
        num_in_row: 1,
        num_in_col: 10,
        w_h: (400.0, 41.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
}
