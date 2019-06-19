#[cfg(feature="_ws")]
pub use conrod_chat::backend::websocket::client::{Message, OwnedMessage};
pub use hardback_meta as meta;
pub use hardback_codec as codec_lib;
use conrod_core;
use conrod_core::text::FontCollection;
use rodio;
use support;
use crayon::prelude::*;
use crayon_audio::prelude::*;
use crayon_bytes::prelude::*;
use hardback_meta::app::{Font,Sprite,ResourceEnum,Texture};
SupportIdType!{}
WindowResources!{
    (a1,ResourceEnum::Sprite(Sprite::RUST),"image","res:rust.png"),
    (a2,ResourceEnum::Sprite(Sprite::UNOFFICIAL),"image","res:unofficial.png"),        
    (a3,ResourceEnum::Sprite(Sprite::CLOUDY),"image","res:cloudy.png"),
    (a4,ResourceEnum::Sprite(Sprite::COININFO),"image","res:allcoin_info.png"),
    (a5,ResourceEnum::Sprite(Sprite::COININFO270),"image","res:allcoin_info (270).png"),
    (a6,ResourceEnum::Sprite(Sprite::GAMEICONS),"image","res:gameicon.png"),
    (a7,ResourceEnum::Font(Font::REGULAR),"font","res:NotoSans-Regular.ttf"),
    (a8,ResourceEnum::Font(Font::BOLD),"font","res:NotoSans-Bold.ttf"),
    (a9,ResourceEnum::Font(Font::BOLDITALIC),"font","res:NotoSans-BoldItalic.ttf"),
    (a10,ResourceEnum::Font(Font::ITALIC),"font","res:NotoSans-Italic.ttf"),
    (a11,ResourceEnum::Font(Font::MYSTERY),"font","res:MysteryQuest-Regular.ttf"),
    (a12,ResourceEnum::Font(Font::HORROR),"font","res:Mortified.ttf"),
    (a13,ResourceEnum::Font(Font::ADVENTURE),"font","res:TradeWinds-Regular.ttf"),
    (a14,ResourceEnum::Font(Font::ROMANCE),"font","res:Babylove.ttf"),
    (a15,ResourceEnum::Texture(Texture::PAGE1F),"texture","res:player1.jpg"),
}
