#![allow(unused_imports)]
#![recursion_limit="128"]
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate conrod_core;
#[macro_use]
extern crate conrod_derive;
#[macro_use]
extern crate cardgame_macros;
extern crate conrod_crayon;
extern crate conrod_chat;
extern crate cardgame_widgets;
extern crate crayon;
extern crate crayon_audio;
extern crate crayon_bytes;
pub extern crate hardback_meta;
pub extern crate hardback_codec;
extern crate futures;
extern crate image;
extern crate rodio;
extern crate chrono;
extern crate instant;
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
