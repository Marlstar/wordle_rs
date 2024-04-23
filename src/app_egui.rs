use eframe::{self, Frame, NativeOptions, run_native, Storage};
use egui::{self, Context, Ui};
use serde::{Deserialize, Serialize};

fn main_egui() -> eframe::Result<()> {
    let mut win_option = NativeOptions {
        centered: true,
        vsync: true,
        ..Default::default()
    };

    run_native(
        "Wordle",
        win_option,
        Box::new(|cc| Box::new(WordleApp::new(cc)))
    )
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WordleApp {
    word: String
}

impl Default for WordleApp {
    fn default() -> Self {
        Self {
            word: String::from("hello")
        }
    }
}

impl WordleApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        return Default::default();
    }
}

impl eframe::App for WordleApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        ctx.set_pixels_per_point(2.0);
        egui::CentralPanel::default()
            .show(ctx,
                  |ui: &mut Ui| {
                      // Word
                      ui.label(format!("{}", self.word.as_str()));

                      // Add letter button
                      if ui.button("Add letter").clicked() {
                          self.word = String::from(format!("{}+", self.word))
                      }
                  }
            );
    }
}