use {
    crate::gui::{palette::dark::DARK, styles::container::Container, widget::row::Row},
    iced::{
        Length,
        widget::{container, row, space},
    },
};

pub fn bar<'a, Message: 'a>(height: Length) -> Row<'a, Message> {
    row![
        container(space())
            .class(Container::Color(DARK.background, false))
            .width(Length::Fill)
            .height(height),
        container(space())
            .class(Container::Color(DARK.primary, false))
            .width(Length::Fill)
            .height(height),
        container(space())
            .class(Container::Color(DARK.success, false))
            .width(Length::Fill)
            .height(height),
        container(space())
            .class(Container::Color(DARK.warning, false))
            .width(Length::Fill)
            .height(height),
        container(space())
            .class(Container::Color(DARK.danger, false))
            .width(Length::Fill)
            .height(height)
    ]
    .into()
}
