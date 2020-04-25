use iced::{Align, Application, Column, Command, Container, Element, Length, Text};
use iced_futures::executor::AsyncStd;

pub struct Clock {}

#[derive(Debug, Clone)]
pub enum Message {
    Toggle,
    Reset,
}

impl Application for Clock {
    type Executor = AsyncStd;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Clock {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Limit Screen Time")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let duration = Text::new("00:00:00").size(40);

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(duration);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
