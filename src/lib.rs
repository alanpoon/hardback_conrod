#![allow(unused_imports)]
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;
#[macro_use]
pub extern crate glium;
#[macro_use]
extern crate cardgame_macros;
extern crate conrod_chat;
extern crate conrod_keypad;
extern crate cardgame_widgets;
pub extern crate hardback_meta;
pub extern crate hardback_codec;
extern crate futures;
extern crate image;
#[cfg(target_os="android")]
extern crate rusttype;
#[cfg(target_os="android")]
extern crate android_glue;
#[cfg(not(target_os="android"))]
extern crate find_folder;
pub mod backend;
pub mod page_curl;
pub mod logic;
pub mod opengl;
pub mod app;
pub mod ui;
pub mod graphics_match;
pub mod on_request;
pub mod support;
pub mod custom_widget;
pub mod instruction;
