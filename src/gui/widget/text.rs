use {
    crate::gui::{element::Renderer, theme::Theme},
    iced::widget,
};

pub type Text<'a> = widget::Text<'a, Theme, Renderer>;
