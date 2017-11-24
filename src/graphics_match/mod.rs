use cardgame_widgets::sprite::SpriteInfo;
pub mod button {
    use graphics_match::SpriteInfo;
    pub fn get_style() -> SpriteInfo {
        SpriteInfo {
            first: (0.0, 535.0),
            num_in_row: 3,
            num_in_col: 11,
            w_h: (180.0, 60.0),
            pad: (0.0, 10.0, 0.0, 10.0),
        }
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
        first: (0.0, 800.0),
        num_in_row: 3,
        num_in_col: 3,
        w_h: (400.0, 400.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
}
