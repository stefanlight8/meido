use crate::gui::views::{about::message::AboutMessage, scanner::message::ScannerMessage};

#[derive(Debug, Clone)]
pub enum Message {
    Scanner(ScannerMessage),
    About(AboutMessage),
}
