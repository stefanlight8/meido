use {
    crate::gui::{
        message::Message, palette::dark::DARK, styles::container::Container, widget::row::Row,
    },
    iced::{
        Length,
        widget::{container, row, space},
    },
};

pub fn bar<'a>(height: Length) -> Row<'a, Message> {
    row![
        container(space())
            .class(Container::Color(DARK.background))
            .width(Length::Fill)
            .height(height),
        container(space())
            .class(Container::Color(DARK.primary))
            .width(Length::Fill)
            .height(height),
        container(space())
            .class(Container::Color(DARK.success))
            .width(Length::Fill)
            .height(height),
        container(space())
            .class(Container::Color(DARK.warning))
            .width(Length::Fill)
            .height(height),
        container(space())
            .class(Container::Color(DARK.danger))
            .width(Length::Fill)
            .height(height)
    ]
    .into()
}
