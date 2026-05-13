use {
    crate::gui::{fonts::inter::INTER_BOLD, styles::text::Text, widget::row::Row},
    iced::{
        Length,
        alignment::Vertical,
        widget::{row, space, text},
    },
};

pub fn status_bar<'a, Message: 'a>(title: &'a str) -> Row<'a, Message> {
    row![
        space().width(Length::Fill),
        text(title).class(Text::Secondary).font(INTER_BOLD).size(12)
    ]
    .align_y(Vertical::Bottom)
    .width(Length::Fill)
    .height({
        #[cfg(target_os = "macos")]
        {
            20
        }

        #[cfg(not(target_os = "macos"))]
        {
            0
        }
    })
    .padding(8)
    .into()
}
