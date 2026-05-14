use {
    crate::gui::{
        element::Element,
        styles::{button::Button, text::Text},
        views::scanner::{
            message::ScannerMessage,
            state::{ScannerState, ScanningState},
        },
        widget::progress_bar,
    },
    iced::{
        Length,
        alignment::Vertical,
        widget::{button, column, row, space, text},
    },
};

pub fn control_panel<'a>(state: &'a ScannerState) -> Element<'a, ScannerMessage> {
    match &state.scanning {
        ScanningState::None => button("Scan").on_press(ScannerMessage::Scan).into(),

        ScanningState::Scanning {
            nodes_index,
            scanned,
            ..
        } => progress_bar(0.0..=1.0, scanned.len() as f32 / nodes_index.len() as f32)
            .girth(3.0)
            .status("Scanning".to_string())
            .details(format!("{}/{}", scanned.len(), nodes_index.len()))
            .view()
            .into(),

        ScanningState::Complete { files_count } => row![
            column![
                text(format!(
                    "{} collected categories",
                    state.trash_buffer.categories().size_hint().0
                ))
                .size(14)
                .class(Text::Secondary),
                text(format!("{} files found", files_count))
                    .size(14)
                    .class(Text::Secondary)
            ],
            space().width(Length::Fill),
            button("Trash").class(Button::Warning),
            button("Delete").class(Button::Danger),
        ]
        .spacing(4)
        .align_y(Vertical::Bottom)
        .into(),
    }
}
