// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// COSMIC desktop UI, using the ported Microsoft Calculator engine.

use cosmic::app::{Core, Task};
use cosmic::iced::alignment::Horizontal;
use cosmic::iced::keyboard::Key;
use cosmic::iced::{event, keyboard, Alignment, Length, Subscription};
use cosmic::widget::{self, button, column, container, row};
use cosmic::{theme, Application, Element};

use crate::engine::command::*;
use crate::engine::{CalcDisplay, CalcEngine};
use crate::fl;

pub const APP_ID: &str = "dev.eagle787sf.CosmicCalculator";

/// Implements the ICalcDisplay trait for bridging the engine to the UI.
struct DisplayBridge {
    primary_display: String,
    is_error: bool,
    paren_count: u32,
}

impl DisplayBridge {
    fn new() -> Self {
        Self {
            primary_display: "0".to_string(),
            is_error: false,
            paren_count: 0,
        }
    }
}

impl CalcDisplay for DisplayBridge {
    fn set_primary_display(&mut self, text: &str, is_error: bool) {
        self.primary_display = text.to_string();
        self.is_error = is_error;
    }

    fn set_is_in_error(&mut self, is_error: bool) {
        self.is_error = is_error;
    }

    fn set_expression_display(&mut self, _tokens: &[(String, i32)]) {}

    fn set_parenthesis_number(&mut self, count: u32) {
        self.paren_count = count;
    }

    fn max_digits_reached(&mut self) {}
    fn binary_operator_received(&mut self) {}
    fn on_no_right_paren_added(&mut self) {}
}

pub struct Calculator {
    core: Core,
    engine: CalcEngine,
    display_bridge: DisplayBridge,
    display_text: String,
    expression_text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    EngineCommand(u32),
    KeyboardEvent(keyboard::Key, keyboard::Modifiers),
}

impl Calculator {
    fn send_command(&mut self, cmd: u32) {
        self.engine.process_command(cmd, &mut self.display_bridge);
        self.display_text = self.display_bridge.primary_display.clone();
        self.expression_text = self.engine.get_expression().to_string();
    }
}

impl Application for Calculator {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = Calculator {
            core,
            engine: CalcEngine::new(true, 16), // Standard mode with precedence, 16-digit precision
            display_bridge: DisplayBridge::new(),
            display_text: "0".to_string(),
            expression_text: String::new(),
        };
        (app, Task::none())
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        vec![widget::text::heading(fl!("app-title")).into()]
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;

        // Expression preview (secondary display)
        let expression = container(
            widget::text(&self.expression_text)
                .size(14.0)
                .width(Length::Fill)
                .align_x(Horizontal::Right),
        )
        .padding([4, 20])
        .width(Length::Fill);

        // Main display
        let display = container(
            widget::text(&self.display_text)
                .size(36.0)
                .width(Length::Fill)
                .align_x(Horizontal::Right),
        )
        .padding([8, 20])
        .width(Length::Fill)
        .height(Length::FillPortion(2));

        // Button helpers using button::custom for full sizing
        let btn =
            |label: &str, cmd: u32, style: theme::Button| -> Element<Message> {
                button::custom(
                    container(widget::text(label.to_string()).size(18.0))
                        .center(Length::Fill)
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .class(style)
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .on_press(Message::EngineCommand(cmd))
                .into()
            };

        let num = |label: &str, cmd: u32| btn(label, cmd, theme::Button::Standard);
        let op = |label: &str, cmd: u32| btn(label, cmd, theme::Button::Suggested);
        let func = |label: &str, cmd: u32| btn(label, cmd, theme::Button::Standard);
        let clear = |label: &str, cmd: u32| btn(label, cmd, theme::Button::Destructive);

        let sp = spacing.space_xs;

        // Standard mode layout matching Microsoft Calculator XAML:
        // CalculatorStandardOperators.xaml + NumberPad.xaml
        //
        // Row 0: [%]    [CE]   [C]    [⌫]
        // Row 1: [1/x]  [x²]   [√x]   [÷]
        // Row 2: [7]    [8]    [9]    [×]
        // Row 3: [4]    [5]    [6]    [−]
        // Row 4: [1]    [2]    [3]    [+]
        // Row 5: [±]    [0]    [.]    [=]

        let row0 = row::with_capacity(4)
            .push(func("%", IDC_PERCENT))
            .push(clear("CE", IDC_CENTR))
            .push(clear("C", IDC_CLEAR))
            .push(func("\u{232B}", IDC_BACK))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row1 = row::with_capacity(4)
            .push(func("\u{215F}x", IDC_REC))   // 1/x
            .push(func("x\u{00B2}", IDC_SQR))   // x²
            .push(func("\u{221A}x", IDC_SQRT))   // √x
            .push(op("\u{00F7}", IDC_DIV))        // ÷
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row2 = row::with_capacity(4)
            .push(num("7", Command::Command7 as u32))
            .push(num("8", Command::Command8 as u32))
            .push(num("9", Command::Command9 as u32))
            .push(op("\u{00D7}", IDC_MUL))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row3 = row::with_capacity(4)
            .push(num("4", Command::Command4 as u32))
            .push(num("5", Command::Command5 as u32))
            .push(num("6", Command::Command6 as u32))
            .push(op("\u{2212}", IDC_SUB))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row4 = row::with_capacity(4)
            .push(num("1", Command::Command1 as u32))
            .push(num("2", Command::Command2 as u32))
            .push(num("3", Command::Command3 as u32))
            .push(op("+", IDC_ADD))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row5 = row::with_capacity(4)
            .push(func("\u{00B1}", IDC_SIGN))     // ±
            .push(num("0", Command::Command0 as u32))
            .push(num(".", IDC_PNT))
            .push(op("=", IDC_EQU))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let buttons = column::with_capacity(6)
            .push(row0)
            .push(row1)
            .push(row2)
            .push(row3)
            .push(row4)
            .push(row5)
            .width(Length::Fill)
            .height(Length::FillPortion(5))
            .spacing(sp);

        column::with_capacity(3)
            .push(expression)
            .push(display)
            .push(buttons)
            .align_x(Alignment::Center)
            .spacing(spacing.space_xxs)
            .padding(spacing.space_xxs)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::EngineCommand(cmd) => {
                self.send_command(cmd);
            }
            Message::KeyboardEvent(key, _modifiers) => {
                if let Some(cmd) = key_to_command(&key) {
                    self.send_command(cmd);
                }
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen_with(|event, _status, _id| match event {
            cosmic::iced::Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modifiers,
                ..
            }) => Some(Message::KeyboardEvent(key, modifiers)),
            _ => None,
        })
    }
}

/// Map keyboard keys to engine commands.
/// Ported from StandardCalculatorViewModel::MapCharacterToButtonId.
fn key_to_command(key: &keyboard::Key) -> Option<u32> {
    match key {
        Key::Character(c) => match c.as_str() {
            "0" => Some(Command::Command0 as u32),
            "1" => Some(Command::Command1 as u32),
            "2" => Some(Command::Command2 as u32),
            "3" => Some(Command::Command3 as u32),
            "4" => Some(Command::Command4 as u32),
            "5" => Some(Command::Command5 as u32),
            "6" => Some(Command::Command6 as u32),
            "7" => Some(Command::Command7 as u32),
            "8" => Some(Command::Command8 as u32),
            "9" => Some(Command::Command9 as u32),
            "+" => Some(IDC_ADD),
            "-" => Some(IDC_SUB),
            "*" => Some(IDC_MUL),
            "/" => Some(IDC_DIV),
            "%" => Some(IDC_PERCENT),
            "." | "," => Some(IDC_PNT),
            "=" => Some(IDC_EQU),
            "(" => Some(IDC_OPENP),
            ")" => Some(IDC_CLOSEP),
            "^" => Some(IDC_PWR),
            "!" => Some(IDC_FAC),
            "r" | "R" => Some(IDC_REC),
            "q" | "Q" => Some(IDC_SQRT),
            _ => None,
        },
        Key::Named(keyboard::key::Named::Enter) => Some(IDC_EQU),
        Key::Named(keyboard::key::Named::Backspace) => Some(IDC_BACK),
        Key::Named(keyboard::key::Named::Escape) => Some(IDC_CLEAR),
        Key::Named(keyboard::key::Named::Delete) => Some(IDC_CENTR),
        _ => None,
    }
}
