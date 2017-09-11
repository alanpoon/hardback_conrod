use cardgame_widgets::sprite::SpriteInfo;
pub mod button {
    use graphics_match::SpriteInfo;
    pub fn get_style() -> SpriteInfo {
        SpriteInfo {
            first: (0.0, 535.0),
            num_in_row: 3.0,
            w_h: (180.0, 60.0),
            pad: (0.0, 10.0, 0.0, 10.0),
        }
    }
}
