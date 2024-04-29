use crate::wordle_backend::prelude::*;
use crate::SizedString;
use iced::{widget::{
    Row, Column,
    Text, text,
    Button, button,
    Container, container
}, border::{self, Radius}, mouse::Cursor, Background, Settings, Sandbox, Element, Border, Length, Color, Theme};
use iced::widget::button::Appearance;

pub const CORNER_ROUNDING_RADIUS: f32 = 10.0;

#[derive(Debug, Clone)]
pub struct WordleApp {
    backend: WordleBackend
}
impl Sandbox for WordleApp {
    type Message = WordleAppMessage;

    fn new() -> Self {
        return Self {
            backend: WordleBackend::from_starter_word(SizedString::<5>::new("hello")) // TODO: Replace with a random word
        }
    }

    fn title(&self) -> String {
        return String::from("Wordle!");
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            _ => ()
        };
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Column::new()
            .push(
                ui_elements::wordle_letter(
                    WordleLetterResult::from('H', WordleLetterColour::Green)
                )
            )
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum WordleAppMessage {

}

struct ColouredButton {
    colour: Color
}
impl button::StyleSheet for ColouredButton {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        button::Appearance {
            background: Some(Background::Color(self.colour)),
            ..Default::default()
        }
    }
}

// static GREEN: ButtonTheme = ButtonTheme {
//     colour: Color::new(0.0,1.0,1.0,1.0)
// };
mod ui_elements {
    use iced::alignment::{Horizontal, Vertical};
    use std::collections::HashMap;
    use super::*;

    mod colours {
        use iced::Color;
        pub const GREEN: Color = Color::from_rgba(91.0, 142.0, 76.0, 100.0);
        pub const YELLOW: Color = Color::from_rgba(176.0, 160.0, 57.0, 100.0);
        pub const GRAY: Color = Color::from_rgba(58.0, 58.0, 60.0, 100.0);
    }

    pub fn wordle_letter<'a>(info: WordleLetterResult) -> Button<'a, WordleAppMessage> {
        const TEST_COLOUR: Color = Color::from_rgba(91.0,142.0,76.0,100.0);
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
            _ => Color::from_rgba(1.0,0.0,0.0,1.0)
        };

        // let style = ColouredButton{colour: des_colour};
        let style = ColouredButton{colour: TEST_COLOUR};
        let button = button(text)
            .width(Length::from(SQUARE_BUTTON_SIZE))
            .height(Length::from(SQUARE_BUTTON_SIZE))
            .style(iced::theme::Button::Custom(Box::new(style)));
        return button;
    }
}