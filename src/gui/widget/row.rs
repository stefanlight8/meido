use {
    crate::gui::{element::Renderer, theme::Theme},
    iced::widget,
};

pub type Row<'a, Message> = widget::Row<'a, Message, Theme, Renderer>;
