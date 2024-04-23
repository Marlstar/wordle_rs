#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_iced;
pub mod sized_string; use sized_string::SizedString;

fn main() {
    println!("There are {} l's in the word 'hello'", SizedString::<5>::new("hello").count_letter('l'));
}