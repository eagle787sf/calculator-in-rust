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
    mode: CalcMode,
    is_inv: bool,
    angle_type: AngleDisplay,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalcMode {
    Standard,
    Scientific,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleDisplay {
    Deg,
    Rad,
    Grad,
}

impl AngleDisplay {
    fn label(self) -> &'static str {
        match self {
            AngleDisplay::Deg => "DEG",
            AngleDisplay::Rad => "RAD",
            AngleDisplay::Grad => "GRAD",
        }
    }
    fn next(self) -> Self {
        match self {
            AngleDisplay::Deg => AngleDisplay::Rad,
            AngleDisplay::Rad => AngleDisplay::Grad,
            AngleDisplay::Grad => AngleDisplay::Deg,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EngineCommand(u32),
    KeyboardEvent(keyboard::Key, keyboard::Modifiers),
    SwitchMode(CalcMode),
    ToggleInv,
    CycleAngleType,
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
            engine: CalcEngine::new(true, 16), // Precedence enabled, 16-digit precision
            display_bridge: DisplayBridge::new(),
            display_text: "0".to_string(),
            expression_text: String::new(),
            mode: CalcMode::Standard,
            is_inv: false,
            angle_type: AngleDisplay::Deg,
        };
        (app, Task::none())
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        let title = widget::text::heading(fl!("app-title"));

        // Mode switcher: Standard | Scientific
        let std_style = if self.mode == CalcMode::Standard {
            theme::Button::Suggested
        } else {
            theme::Button::Text
        };
        let sci_style = if self.mode == CalcMode::Scientific {
            theme::Button::Suggested
        } else {
            theme::Button::Text
        };
        let std_btn = button::custom(widget::text("Standard").size(14.0))
            .class(std_style)
            .on_press(Message::SwitchMode(CalcMode::Standard));
        let sci_btn = button::custom(widget::text("Scientific").size(14.0))
            .class(sci_style)
            .on_press(Message::SwitchMode(CalcMode::Scientific));

        vec![
            title.into(),
            cosmic::iced::widget::Space::new().width(Length::Fixed(12.0)).into(),
            std_btn.into(),
            sci_btn.into(),
        ]
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

        let btn_msg =
            |label: &str, msg: Message, style: theme::Button| -> Element<Message> {
                button::custom(
                    container(widget::text(label.to_string()).size(18.0))
                        .center(Length::Fill)
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .class(style)
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .on_press(msg)
                .into()
            };

        let num = |label: &str, cmd: u32| btn(label, cmd, theme::Button::Standard);
        let op = |label: &str, cmd: u32| btn(label, cmd, theme::Button::Suggested);
        let func = |label: &str, cmd: u32| btn(label, cmd, theme::Button::Standard);
        let clear = |label: &str, cmd: u32| btn(label, cmd, theme::Button::Destructive);

        let sp = spacing.space_xs;

        let buttons = match self.mode {
            CalcMode::Standard => self.build_standard_buttons(&num, &op, &func, &clear, sp),
            CalcMode::Scientific => self.build_scientific_buttons(&num, &op, &func, &clear, &btn_msg, sp),
        };

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
                if self.is_inv && is_invertible_unary_op(cmd) {
                    self.engine.process_command(IDC_INV, &mut self.display_bridge);
                }
                self.send_command(cmd);
                if self.is_inv && is_invertible_unary_op(cmd) {
                    self.is_inv = false;
                }
            }
            Message::KeyboardEvent(key, _modifiers) => {
                if let Some(cmd) = key_to_command(&key) {
                    self.send_command(cmd);
                }
            }
            Message::SwitchMode(new_mode) => {
                self.mode = new_mode;
                self.is_inv = false;
            }
            Message::ToggleInv => {
                self.is_inv = !self.is_inv;
            }
            Message::CycleAngleType => {
                self.angle_type = self.angle_type.next();
                let cmd = match self.angle_type {
                    AngleDisplay::Deg => 321,
                    AngleDisplay::Rad => 322,
                    AngleDisplay::Grad => 323,
                };
                self.engine.process_command(cmd, &mut self.display_bridge);
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

impl Calculator {
    fn build_standard_buttons<'a, FN, FO, FF, FC>(
        &self,
        num: &FN,
        op: &FO,
        func: &FF,
        clear: &FC,
        sp: u16,
    ) -> Element<'a, Message>
    where
        FN: Fn(&str, u32) -> Element<'a, Message>,
        FO: Fn(&str, u32) -> Element<'a, Message>,
        FF: Fn(&str, u32) -> Element<'a, Message>,
        FC: Fn(&str, u32) -> Element<'a, Message>,
    {
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
            .push(func("\u{215F}x", IDC_REC))
            .push(func("x\u{00B2}", IDC_SQR))
            .push(func("\u{221A}x", IDC_SQRT))
            .push(op("\u{00F7}", IDC_DIV))
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
            .push(func("\u{00B1}", IDC_SIGN))
            .push(num("0", Command::Command0 as u32))
            .push(num(".", IDC_PNT))
            .push(op("=", IDC_EQU))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        column::with_capacity(6)
            .push(row0)
            .push(row1)
            .push(row2)
            .push(row3)
            .push(row4)
            .push(row5)
            .width(Length::Fill)
            .height(Length::FillPortion(5))
            .spacing(sp)
            .into()
    }

    fn build_scientific_buttons<'a, FN, FO, FF, FC, FM>(
        &self,
        num: &FN,
        op: &FO,
        func: &FF,
        clear: &FC,
        btn_msg: &FM,
        sp: u16,
    ) -> Element<'a, Message>
    where
        FN: Fn(&str, u32) -> Element<'a, Message>,
        FO: Fn(&str, u32) -> Element<'a, Message>,
        FF: Fn(&str, u32) -> Element<'a, Message>,
        FC: Fn(&str, u32) -> Element<'a, Message>,
        FM: Fn(&str, Message, theme::Button) -> Element<'a, Message>,
    {
        // Scientific mode layout matching Microsoft Calculator's Scientific mode.
        //
        // Row 0: [DEG/RAD/GRAD] [F-E]   [(]    [)]
        // Row 1: [INV]   [π]    [e]    [C]    [⌫]  -> split into 2 rows
        // Using 5-column layout like Windows Calculator Scientific:
        //
        // Row 0: [DEG]    [F-E]   [(]    [)]    [⌫]
        // Row 1: [x²]     [x^y]   [sin]  [cos]  [tan]
        // Row 2: [√x]     [10^x]  [log]  [ln]   [INV]
        // Row 3: [x!]     [±]     [π]    [e]    [÷]
        // Row 4: [7]      [8]     [9]    [×]    [%]
        // Row 5: [4]      [5]     [6]    [−]    [1/x]
        // Row 6: [1]      [2]     [3]    [+]    [abs]
        // Row 7: [CE]     [0]     [.]    [=]    [C]

        let inv_style = if self.is_inv {
            theme::Button::Suggested
        } else {
            theme::Button::Standard
        };

        let row0 = row::with_capacity(5)
            .push(btn_msg(self.angle_type.label(), Message::CycleAngleType, theme::Button::Standard))
            .push(func("F-E", IDC_FE))
            .push(func("(", IDC_OPENP))
            .push(func(")", IDC_CLOSEP))
            .push(func("\u{232B}", IDC_BACK))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row1 = row::with_capacity(5)
            .push(func("x\u{00B2}", IDC_SQR))
            .push(op("x^y", IDC_PWR))
            .push(func(if self.is_inv { "sin\u{207B}\u{00B9}" } else { "sin" }, IDC_SIN))
            .push(func(if self.is_inv { "cos\u{207B}\u{00B9}" } else { "cos" }, IDC_COS))
            .push(func(if self.is_inv { "tan\u{207B}\u{00B9}" } else { "tan" }, IDC_TAN))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row2 = row::with_capacity(5)
            .push(func("\u{221A}x", IDC_SQRT))
            .push(func("10\u{02E3}", IDC_POW10))
            .push(func("log", IDC_LOG))
            .push(func("ln", IDC_LN))
            .push(btn_msg("INV", Message::ToggleInv, inv_style))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row3 = row::with_capacity(5)
            .push(func("n!", IDC_FAC))
            .push(func("\u{00B1}", IDC_SIGN))
            .push(func("\u{03C0}", IDC_PI))
            .push(func("e", IDC_EULER))
            .push(op("\u{00F7}", IDC_DIV))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row4 = row::with_capacity(5)
            .push(num("7", Command::Command7 as u32))
            .push(num("8", Command::Command8 as u32))
            .push(num("9", Command::Command9 as u32))
            .push(op("\u{00D7}", IDC_MUL))
            .push(func("%", IDC_PERCENT))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row5 = row::with_capacity(5)
            .push(num("4", Command::Command4 as u32))
            .push(num("5", Command::Command5 as u32))
            .push(num("6", Command::Command6 as u32))
            .push(op("\u{2212}", IDC_SUB))
            .push(func("\u{215F}x", IDC_REC))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row6 = row::with_capacity(5)
            .push(num("1", Command::Command1 as u32))
            .push(num("2", Command::Command2 as u32))
            .push(num("3", Command::Command3 as u32))
            .push(op("+", IDC_ADD))
            .push(func("|x|", IDC_ABS))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        let row7 = row::with_capacity(5)
            .push(clear("CE", IDC_CENTR))
            .push(num("0", Command::Command0 as u32))
            .push(num(".", IDC_PNT))
            .push(op("=", IDC_EQU))
            .push(clear("C", IDC_CLEAR))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        column::with_capacity(8)
            .push(row0)
            .push(row1)
            .push(row2)
            .push(row3)
            .push(row4)
            .push(row5)
            .push(row6)
            .push(row7)
            .width(Length::Fill)
            .height(Length::FillPortion(5))
            .spacing(sp)
            .into()
    }
}

/// Returns true if this unary op has an inverse (toggled with INV).
/// Matches the reset-INV list in scicomm.cpp ProcessCommandWorker.
fn is_invertible_unary_op(op: u32) -> bool {
    matches!(
        op,
        IDC_SIN | IDC_COS | IDC_TAN
            | IDC_SINH | IDC_COSH | IDC_TANH
            | IDC_SEC | IDC_CSC | IDC_COT
            | IDC_SECH | IDC_CSCH | IDC_COTH
            | IDC_LN | IDC_CHOP
    )
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
