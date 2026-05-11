use {
    crate::gui::{element::Renderer, theme::Theme},
    iced::widget,
};

pub type Column<'a, Message> = widget::Column<'a, Message, Theme, Renderer>;
