use crate::error::CatchAllError;
use crate::game::Game;
use crate::game::Turn;
use crate::piece::Color;
use crate::position::Position;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, Button, Column, Container, Row, Text};
use iced::{theme, Alignment, Element, Length, Renderer, Sandbox, Theme};

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        iced::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    };
}

const LIGHT_SQUARE: iced::Color = rgb!(240, 217, 181);
const DARK_SQUARE: iced::Color = rgb!(181, 136, 99);
const HIGHLIGHTED_SQUARE: iced::Color = rgb!(255, 0, 0);

pub trait Decorate {
    type Output;
    fn decorate(self) -> Self::Output;
}

impl<'a> Decorate for Text<'a, Renderer> {
    type Output = Text<'a, Renderer>;
    fn decorate(self) -> Self::Output {
        self.horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .size(75)
    }
}

impl<'a> Decorate for Button<'a, Message, Renderer> {
    type Output = Button<'a, Message, Renderer>;
    fn decorate(self) -> Self::Output {
        self.height(100).width(100)
    }
}

impl<'a> Decorate for Container<'a, Message, Renderer> {
    type Output = Container<'a, Message, Renderer>;
    fn decorate(self) -> Self::Output {
        self.width(Length::Shrink).height(Length::Shrink)
    }
}

impl<'a> Decorate for Row<'a, Message, Renderer> {
    type Output = Row<'a, Message, Renderer>;
    fn decorate(self) -> Self::Output {
        self.align_items(Alignment::Center)
    }
}

impl<'a> Decorate for Column<'a, Message, Renderer> {
    type Output = Column<'a, Message, Renderer>;
    fn decorate(self) -> Self::Output {
        self.align_items(Alignment::Center)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Move(Position),
}

struct Square {
    position: Position,
    turn: Turn,
}

impl Square {
    fn new(position: Position, turn: Turn) -> Self {
        Self { position, turn }
    }
}

impl button::StyleSheet for Square {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        let color = match self.turn {
            Turn::Select(_, pos) if self.position.eq(&pos) => HIGHLIGHTED_SQUARE,
            _ => match Color::from(self.position) {
                Color::White => LIGHT_SQUARE,
                Color::Black => DARK_SQUARE,
            },
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
    game: Game,
}

impl Gui {
    fn square_view<'a>(&self, pos: Position) -> Button<'a, Message, Renderer> {
        let turn = self.game.turn();
        let theme = theme::Button::custom(Square::new(pos, turn));

        let mut text = Text::new("");

        let res = self.game.at(&pos);

        if let Ok(piece) = res {
            let color = match piece.color() {
                Color::White => iced::Color::WHITE,
                Color::Black => iced::Color::BLACK,
            };
            text = Text::new(piece.to_string()).style(color).decorate();
        }

        button(text)
            .style(theme)
            .decorate()
            .on_press(Message::Move(pos))
    }
}

impl Default for Gui {
    fn default() -> Self {
        Self { game: Game::new() }
    }
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

        if let Err(e) = self.game.advance(&pos) {
            println!("{}", e);
            self.game.reset_turn();

            if let CatchAllError::NoLegalMoves = e {
                self.game.reset();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut column = Column::new().decorate();
        for rank in (0..8).rev() {
            let mut row = Row::new().decorate();
            for file in 0..8 {
                let button = self.square_view(Position::new(file, rank));
                row = row.push(button);
            }
            column = column.push(row);
        }

        Container::new(column).decorate().into()
    }
}
