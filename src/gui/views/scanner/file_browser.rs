use {
    crate::gui::{
        element::Element,
        views::scanner::{message::ScannerMessage, state::ScannerState},
    },
    iced::widget::{checkbox, column, row, text},
    std::path::PathBuf,
};

pub fn file_browser<'a>(
    state: &'a ScannerState,
    files: impl Iterator<Item = &'a PathBuf> + 'a,
) -> Element<'a, ScannerMessage> {
    column(files.map(|file| {
        row![
            checkbox(state.trash_buffer.contains(&file)),
            text(file.to_str().unwrap_or("Unknown"))
        ]
        .spacing(4)
        .into()
    }))
    .into()
}
