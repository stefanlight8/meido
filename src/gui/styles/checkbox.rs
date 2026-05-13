use {
    crate::gui::theme::Theme,
    iced::{
        Background,
        border::{Border, radius},
        theme::{Base, palette::deviate},
        widget::checkbox::{Catalog, Status, Style},
    },
};

pub struct Checkbox;

impl Catalog for Theme {
    type Class<'a> = Checkbox;

    fn default<'a>() -> Self::Class<'a> {
        Checkbox
    }

    fn style(&self, _: &Self::Class<'_>, status: Status) -> Style {
        let palette = self.palette().unwrap();

        Style {
            background: Background::Color(match status {
                Status::Active { is_checked: true } => palette.primary,
                Status::Active { is_checked: false } => palette.background,
                Status::Hovered { .. } => deviate(palette.primary, 0.50),
                Status::Disabled { is_checked: true } => deviate(palette.primary, 0.50),
                Status::Disabled { is_checked: false } => palette.background,
            }),
            icon_color: palette.background,
            border: Border {
                color: deviate(palette.primary, 0.40),
                width: 0.1,
                radius: radius(6),
                ..Default::default()
            },
            text_color: Some(palette.text),
        }
    }
}
