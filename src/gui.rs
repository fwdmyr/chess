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
const HIGHLIGHTED_SQUARE: iced::Color = rgb!(255, 0, 0);

struct Square {
    file: usize,
    rank: usize,
    is_from: bool,
}

impl Square {
    fn new(pos: &Position, from: &Position) -> Self {
        Self {
            file: pos.file(),
            rank: pos.rank(),
            is_from: pos == from,
        }
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
        let color = match self.is_from {
            true => HIGHLIGHTED_SQUARE,
            false => color,
        };

        button::Appearance {
            background: Some(iced::Background::Color(color)),
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(HIGHLIGHTED_SQUARE)),
            ..Default::default()
        }
    }
}

pub struct Gui {
    board: Board,
    from: Position,
    to: Position,
    turn_state: TurnState,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            board: Board::new(),
            from: Position::default(),
            to: Position::default(),
            turn_state: TurnState::From(Color::White),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Move(Position),
}

#[derive(Debug, Clone, Copy)]
pub enum TurnState {
    From(Color),
    To(Color),
}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Chess".to_string()
    }

    fn update(&mut self, msg: Message) {
        let pos = match msg {
            Message::Move(pos) => pos,
        };

        match self.turn_state {
            TurnState::From(_) => {
                self.from = pos;
            }
            TurnState::To(_) => {
                self.to = pos;
            }
        };

        let res = match self.turn_state {
            TurnState::To(color) => self.board.advance(&color, &self.from, &self.to),
            TurnState::From(_) => Ok(()),
        };

        match (res, self.turn_state.clone()) {
            (Ok(_), TurnState::To(color)) => {
                self.turn_state = match color {
                    Color::White => TurnState::From(Color::Black),
                    Color::Black => TurnState::From(Color::White),
                }
            }
            (Err(e), TurnState::To(color)) => {
                self.turn_state = TurnState::From(color);
                println!("{}", e);
            }
            (Ok(_), TurnState::From(color)) => {
                self.turn_state = TurnState::To(color);
            }
            _ => panic!(),
        }
    }

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
                    .style(theme::Button::custom(Square::new(
                        &Position::new(file, rank),
                        &self.from,
                    )))
                    .height(100)
                    .width(100)
                    .on_press(Message::Move(Position::new(file, rank))),
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
