use cosmic::app::{Core, Task};
use cosmic::iced::alignment::Horizontal;
use cosmic::iced::keyboard::Key;
use cosmic::iced::{event, keyboard, Alignment, Length, Subscription};
use cosmic::widget::{self, button, column, container, row, text};
use cosmic::{theme, Application, Element};

use crate::calculator;
use crate::fl;

pub const APP_ID: &str = "dev.eagle787sf.CosmicCalculator";

pub struct Calculator {
    core: Core,
    expression: String,
    display: String,
    last_result: Option<f64>,
    error: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    NumberPress(char),
    OperatorPress(char),
    DecimalPress,
    EqualsPress,
    AllClearPress,
    BackspacePress,
    ParenOpen,
    ParenClose,
    PercentPress,
    PowerPress,
    SqrtPress,
    NegatePress,
    KeyboardEvent(keyboard::Key, keyboard::Modifiers),
}

impl Calculator {
    fn append_to_expression(&mut self, ch: char) {
        if self.error {
            self.expression.clear();
            self.display.clear();
            self.error = false;
        }
        self.expression.push(ch);
        self.display = self.expression.clone();
    }

    fn append_str(&mut self, s: &str) {
        if self.error {
            self.expression.clear();
            self.display.clear();
            self.error = false;
        }
        self.expression.push_str(s);
        self.display = self.expression.clone();
    }

    fn do_evaluate(&mut self) {
        match calculator::evaluate(&self.expression) {
            Ok(result) => {
                let formatted = calculator::format_result(result);
                self.display = formatted.clone();
                self.last_result = Some(result);
                self.expression = formatted;
                self.error = false;
            }
            Err(e) => {
                self.display = e;
                self.error = true;
            }
        }
    }

    fn clear_all(&mut self) {
        self.expression.clear();
        self.display = "0".into();
        self.last_result = None;
        self.error = false;
    }

    fn backspace(&mut self) {
        if self.error {
            self.clear_all();
            return;
        }
        self.expression.pop();
        if self.expression.is_empty() {
            self.display = "0".into();
        } else {
            self.display = self.expression.clone();
        }
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
            expression: String::new(),
            display: "0".into(),
            last_result: None,
            error: false,
        };
        (app, Task::none())
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        vec![text::heading(fl!("app-title")).into()]
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;

        let display_text = if self.display.is_empty() {
            "0"
        } else {
            &self.display
        };

        // Display area - large text, right-aligned
        let display = container(
            widget::text(display_text)
                .size(36.0)
                .width(Length::Fill)
                .align_x(Horizontal::Right),
        )
        .padding([16, 20])
        .width(Length::Fill)
        .height(Length::FillPortion(2));

        // Expression preview (smaller, shows what you're typing)
        let expression_preview = container(
            widget::text(&self.expression)
                .size(16.0)
                .width(Length::Fill)
                .align_x(Horizontal::Right),
        )
        .padding([4, 20])
        .width(Length::Fill);

        // Button helpers using button::custom for full height/width control
        let calc_btn = |label: &str, msg: Message| -> Element<Message> {
            button::custom(
                container(widget::text(label.to_string()).size(20.0))
                    .center(Length::Fill)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .class(theme::Button::Standard)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .on_press(msg)
            .into()
        };

        let calc_btn_accent = |label: &str, msg: Message| -> Element<Message> {
            button::custom(
                container(widget::text(label.to_string()).size(20.0))
                    .center(Length::Fill)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .class(theme::Button::Suggested)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .on_press(msg)
            .into()
        };

        let calc_btn_destructive = |label: &str, msg: Message| -> Element<Message> {
            button::custom(
                container(widget::text(label.to_string()).size(20.0))
                    .center(Length::Fill)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .class(theme::Button::Destructive)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .on_press(msg)
            .into()
        };

        let sp = spacing.space_xs;

        // Row 1: ( ) sqrt ^
        let row1 = row::with_capacity(4)
            .push(calc_btn("(", Message::ParenOpen))
            .push(calc_btn(")", Message::ParenClose))
            .push(calc_btn("\u{221A}", Message::SqrtPress))
            .push(calc_btn("x\u{207F}", Message::PowerPress))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        // Row 2: C % / backspace
        let row2 = row::with_capacity(4)
            .push(calc_btn_destructive("C", Message::AllClearPress))
            .push(calc_btn("%", Message::PercentPress))
            .push(calc_btn_accent("\u{00F7}", Message::OperatorPress('/')))
            .push(calc_btn("\u{232B}", Message::BackspacePress))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        // Row 3: 7 8 9 *
        let row3 = row::with_capacity(4)
            .push(calc_btn("7", Message::NumberPress('7')))
            .push(calc_btn("8", Message::NumberPress('8')))
            .push(calc_btn("9", Message::NumberPress('9')))
            .push(calc_btn_accent("\u{00D7}", Message::OperatorPress('*')))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        // Row 4: 4 5 6 -
        let row4 = row::with_capacity(4)
            .push(calc_btn("4", Message::NumberPress('4')))
            .push(calc_btn("5", Message::NumberPress('5')))
            .push(calc_btn("6", Message::NumberPress('6')))
            .push(calc_btn_accent("\u{2212}", Message::OperatorPress('-')))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        // Row 5: 1 2 3 +
        let row5 = row::with_capacity(4)
            .push(calc_btn("1", Message::NumberPress('1')))
            .push(calc_btn("2", Message::NumberPress('2')))
            .push(calc_btn("3", Message::NumberPress('3')))
            .push(calc_btn_accent("+", Message::OperatorPress('+')))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        // Row 6: +/- 0 . =
        let row6 = row::with_capacity(4)
            .push(calc_btn("\u{00B1}", Message::NegatePress))
            .push(calc_btn("0", Message::NumberPress('0')))
            .push(calc_btn(".", Message::DecimalPress))
            .push(calc_btn_accent("=", Message::EqualsPress))
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(sp);

        // Button area - all rows stacked vertically, filling space
        let buttons = column::with_capacity(6)
            .push(row1)
            .push(row2)
            .push(row3)
            .push(row4)
            .push(row5)
            .push(row6)
            .width(Length::Fill)
            .height(Length::FillPortion(5))
            .spacing(sp);

        // Main layout
        column::with_capacity(3)
            .push(expression_preview)
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
            Message::NumberPress(n) => {
                // If we just evaluated and press a number, start fresh
                if self.last_result.is_some() && !self.error {
                    self.expression.clear();
                    self.last_result = None;
                }
                self.append_to_expression(n);
            }
            Message::OperatorPress(op) => {
                if self.error {
                    return Task::none();
                }
                self.last_result = None;
                self.append_to_expression(op);
            }
            Message::DecimalPress => {
                if self.last_result.is_some() && !self.error {
                    self.expression.clear();
                    self.expression.push('0');
                    self.last_result = None;
                }
                self.append_to_expression('.');
            }
            Message::EqualsPress => {
                self.do_evaluate();
            }
            Message::AllClearPress => {
                self.clear_all();
            }
            Message::BackspacePress => {
                self.backspace();
            }
            Message::ParenOpen => {
                if self.last_result.is_some() {
                    self.expression.clear();
                    self.last_result = None;
                }
                self.append_to_expression('(');
            }
            Message::ParenClose => {
                self.append_to_expression(')');
            }
            Message::PercentPress => {
                self.append_to_expression('%');
            }
            Message::PowerPress => {
                self.last_result = None;
                self.append_to_expression('^');
            }
            Message::SqrtPress => {
                if self.last_result.is_some() {
                    self.expression.clear();
                    self.last_result = None;
                }
                self.append_str("sqrt(");
            }
            Message::NegatePress => {
                if self.error {
                    return Task::none();
                }
                // Toggle negation: wrap expression in -(...)
                if self.expression.starts_with("-(") && self.expression.ends_with(')') {
                    // Remove negation wrapper
                    self.expression = self.expression[2..self.expression.len() - 1].to_string();
                } else if !self.expression.is_empty() {
                    self.expression = format!("-({})", self.expression);
                } else {
                    self.expression = "-".to_string();
                }
                self.display = self.expression.clone();
            }
            Message::KeyboardEvent(key, _modifiers) => {
                if let Some(msg) = Self::key_to_message(&key) {
                    return self.update(msg);
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

impl Calculator {
    fn key_to_message(key: &keyboard::Key) -> Option<Message> {
        match key {
            Key::Character(c) => {
                match c.as_str() {
                    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                        Some(Message::NumberPress(c.as_str().chars().next().unwrap()))
                    }
                    "+" => Some(Message::OperatorPress('+')),
                    "-" => Some(Message::OperatorPress('-')),
                    "*" => Some(Message::OperatorPress('*')),
                    "/" => Some(Message::OperatorPress('/')),
                    "." | "," => Some(Message::DecimalPress),
                    "(" => Some(Message::ParenOpen),
                    ")" => Some(Message::ParenClose),
                    "%" => Some(Message::PercentPress),
                    "^" => Some(Message::PowerPress),
                    "=" => Some(Message::EqualsPress),
                    _ => None,
                }
            }
            Key::Named(keyboard::key::Named::Enter) => Some(Message::EqualsPress),
            Key::Named(keyboard::key::Named::Backspace) => Some(Message::BackspacePress),
            Key::Named(keyboard::key::Named::Escape) => Some(Message::AllClearPress),
            Key::Named(keyboard::key::Named::Delete) => Some(Message::AllClearPress),
            _ => None,
        }
    }
}
