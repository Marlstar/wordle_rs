#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_egui;
mod app_iced;
pub mod sized_string;

use app_egui::WordleApp;

use egui::{
    self,
};
use eframe::{
    self,
    run_native,
    NativeOptions
};

fn main() {

}


