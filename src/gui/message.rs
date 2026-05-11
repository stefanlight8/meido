use crate::gui::views::scanner::message::ScannerMessage;

#[derive(Debug)]
pub enum Message {
    Scanner(ScannerMessage),
}
