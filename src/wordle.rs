use crate::wordle_backend::prelude::*;
use iced::{widget::{
    Row, Column,
    Text, text,
    Button, button,
    Container, container,
    TextInput, text_input,
    Space
}, border::{self, Radius}, mouse::Cursor, Background, Settings, Sandbox, Element, Border, Length, Color, Theme, alignment};
use iced::widget::button::Appearance;
use crate::wordle::ui_elements::wordle_letter;
use crate::wordle_backend::{WORD_SIZE,GUESS_COUNT};

pub const CORNER_ROUNDING_RADIUS: f32 = 10.0;

#[derive(Debug, Clone)]
pub struct WordleApp {
    backend: WordleBackend,
    current_input: String
}
impl Sandbox for WordleApp {
    type Message = WordleAppMessage;

    fn new() -> Self {
        return Self {
            backend: WordleBackend::from_starter_word(String::from("HELLO")), // TODO: Replace with a random word
            current_input: String::new()
        }
    }

    fn title(&self) -> String {
        return String::from("Wordle!");
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            WordleAppMessage::GuessBoxChanged(s) => {
                self.current_input = s.to_ascii_uppercase();
            },
            WordleAppMessage::Guess => {
                self.backend.guess(&self.current_input);
            }
            // _ => ()
        };
    }

    fn view(&self) -> Element<'_, Self::Message> {
        const SPACING: f32 = 5.0;

        let mut master_layout = Column::new();

        // Letters
        master_layout = {
            let guesses: &[WordleGuess<WORD_SIZE>; GUESS_COUNT] = self.backend.guesses();
            let mut rows: Column<WordleAppMessage> = Column::new().push(Space::with_height(SPACING));

            for guess in guesses {
                let mut row: Row<WordleAppMessage> = Row::new().push(Space::with_width(SPACING));
                let guess_results = guess.check(&self.backend.word);

                for letter in guess_results {
                    row = row.push(ui_elements::wordle_letter(letter))
                        .push(Space::with_width(SPACING))
                }
                rows = rows.push(row)
                    .push(Space::with_height(SPACING));
            }

            master_layout = master_layout.push(
                Container::new(rows)
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fill)
            );

            master_layout
        };

        // Text input
        master_layout = {
            let mut ti: TextInput<WordleAppMessage> = text_input("GUESS", self.current_input.as_str())
                .on_input(WordleAppMessage::GuessBoxChanged)
                .on_submit(WordleAppMessage::Guess);

            master_layout.push(ti)
        };

        return master_layout.into();
    }
}

#[derive(Debug, Clone)]
pub enum WordleAppMessage {
    GuessBoxChanged(String),
    Guess
}

struct ColouredButton {
    colour: Color
}
impl button::StyleSheet for ColouredButton {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        const BORDER_RADIUS: f32 = 8.0;

        button::Appearance {
            background: Some(Background::Color(self.colour)),
            border: Border {
                color: Default::default(),
                width: 0.0,
                radius: Radius::from(BORDER_RADIUS),
            },
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance { return self.active(style); }
    fn pressed(&self, style: &Self::Style) -> Appearance { return self.active(style); }
    fn disabled(&self, style: &Self::Style) -> Appearance { return self.active(style); }
}

mod ui_elements {
    use iced::alignment::{Horizontal, Vertical};
    use std::collections::HashMap;
    use super::*;

    mod colours {
        use iced::Color;
        pub const GREEN: Color = Color {r:91.0/255.0, g:142.0/255.0, b:76.0/255.0, a: 1.0};
        pub const YELLOW: Color = Color {r:176.0/255.0, g:160.0/255.0, b:57.0/255.0, a: 1.0};
        pub const GRAY: Color = Color {r:58.0/255.0, g:58.0/255.0, b:60.0/255.0, a: 1.0};
    }

    pub fn wordle_letter<'a>(info: WordleLetterResult) -> Button<'a, WordleAppMessage> {
        // const TEST_COLOUR: Color = Color::from_rgba(91.0,142.0,76.0,100.0);
        const SQUARE_BUTTON_SIZE: u16 = 60;
        const TEXT_SIZE: u16 = 30;

        let text = text(format!("{}", info.letter))
            .size(TEXT_SIZE)
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center);


        let des_colour = match info.result {
            WordleLetterColour::Green => colours::GREEN,
            WordleLetterColour::Yellow => colours::YELLOW,
            WordleLetterColour::Gray => colours::GRAY,
            _ => Color {r:1.0, g:0.0, b:0.0, a:1.0}
        };

        let style = ColouredButton {colour: des_colour};
        // let style = ColouredButton{colour: TEST_COLOUR};
        let button = button(text)
            .width(Length::from(SQUARE_BUTTON_SIZE))
            .height(Length::from(SQUARE_BUTTON_SIZE))
            .style(iced::theme::Button::Custom(Box::new(style)));
        return button;
    }
}