#![feature(once_cell)]

use iced::{
    widget::{button, column, text},
    Alignment, Element, Sandbox, Settings,
};
use xcell_gui::IcedLogger;
mod errors;
mod logger;

pub fn main() -> iced::Result {
    IcedLogger::default().activate().unwrap();
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                log::info!("{}", self.value);
            }
            Message::DecrementPressed => {
                self.value -= 1;
                log::info!("{}", self.value);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
