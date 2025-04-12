use iced::widget::{button, text};
use iced::Element;
use pywal_discord::pywal_discord;

pub mod pywal_discord;

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn update(counter: &mut u64, msg: Message) {
    match msg {
        Message::Increment => *counter += 1,
    }
}

fn view(counter: &u64) -> Element<Message> {
    button(text(counter)).on_press(Message::Increment).into()
}

fn main() -> iced::Result {
    iced::run("counter", update, view)
}
