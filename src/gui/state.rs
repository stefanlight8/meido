use {
    super::message::Message,
    crate::{
        expanders::Expander,
        gui::{
            element::Element,
            theme::Theme,
            views::{about::state::AboutState, scanner::state::ScannerState},
        },
        node::Node,
        rules::Rule,
    },
    iced::Task,
};

enum Screen {
    Scanner,
    About,
}

pub struct State {
    screen: Screen,
    scanner: ScannerState,
    about: AboutState,
}

impl State {
    pub fn new(
        expanders: &'static [&'static dyn Expander<Node>],
        rules: &'static [&'static dyn Rule],
    ) -> Self {
        Self {
            screen: Screen::Scanner,
            scanner: ScannerState::new(expanders, rules),
            about: AboutState,
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::Light
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Scanner(message) => self.scanner.update(message).map(Message::Scanner),
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.screen {
            Screen::Scanner => self.scanner.view().map(Message::Scanner),
            Screen::About => self.about.view().map(Message::About),
        }
    }
}
