#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports)]

mod wordle_backend;
pub mod sized_string; use sized_string::SizedString;
mod wordle;

use iced::{
    Settings,
    Sandbox,
    widget,
    Rectangle
};

fn main() {
    let settings = Settings {
        window: iced::window::Settings {
            size: iced::Size{width: 400.0, height: 600.0},
            resizable: false,
            ..Default::default()
        },

        ..Default::default()
    };

    wordle::WordleApp::run(settings).unwrap();
}