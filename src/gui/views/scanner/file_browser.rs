use {
    crate::gui::{
        element::Element,
        styles::text::Text,
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
        let selected = state.trash_buffer.can.contains(file);
        let deleted = state.deleted_files.contains(file);

        row![
            checkbox(selected || deleted).on_toggle(move |status| {
                let file = file.clone();

                if status || deleted {
                    ScannerMessage::SelectFile(file)
                } else {
                    ScannerMessage::UnselectFile(file)
                }
            }),
            text(file.to_str().unwrap_or("Unknown")).class(if deleted {
                Text::Secondary
            } else {
                Text::Primary
            })
        ]
        .spacing(4)
        .into()
    }))
    .into()
}
