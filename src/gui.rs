use crate::board::Board;
use crate::piece::Color;
use crate::position::Position;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, Column, Container, Row, Text};
use iced::{theme, Alignment, Element, Length, Sandbox, Theme};

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        iced::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    };
}

const LIGHT_SQUARE: iced::Color = rgb!(240, 217, 181);
const DARK_SQUARE: iced::Color = rgb!(181, 136, 99);

struct Square {
    file: usize,
    rank: usize,
}

impl Square {
    fn new(file: usize, rank: usize) -> Self {
        Self { file, rank }
    }
}

impl button::StyleSheet for Square {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        let color = match (self.file + self.rank) % 2 {
            0 => LIGHT_SQUARE,
            1 => DARK_SQUARE,
            _ => panic!(),
        };
        button::Appearance {
            background: Some(iced::Background::Color(color)),
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            ..Default::default()
        }
    }
}

pub struct Gui {
    board: Board,
    fields: [[button::State; 8]; 8],
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            fields: [[button::State::default(); 8]; 8],
            board: Board::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Dummy,
}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "test".to_string()
    }

    fn update(&mut self, msg: Message) {}

    fn view(&self) -> Element<Message> {
        let mut column = Column::new().align_items(Alignment::Center);
        for rank in (0..8).rev() {
            let mut row = Row::new().align_items(Alignment::Center);
            for file in 0..8 {
                let color = self.board.at(&Position::new(file, rank)).map_or(
                    iced::Color::WHITE,
                    |p| match p.color() {
                        Color::White => iced::Color::WHITE,
                        Color::Black => iced::Color::BLACK,
                    },
                );
                row = row.push(
                    button(
                        Text::new(self.board.draw(&Position::new(file, rank)))
                            .horizontal_alignment(Horizontal::Center)
                            .vertical_alignment(Vertical::Center)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .size(75)
                            .style(color),
                    )
                    .style(theme::Button::custom(Square::new(file, rank)))
                    .height(100)
                    .width(100)
                    .on_press(Message::Dummy),
                );
            }
            column = column.push(row);
        }
        Container::new(column)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .into()
    }
}
