use {
    crate::gui::theme::Theme,
    iced::{
        Background, Border,
        border::radius,
        theme::{Base, palette::deviate},
        widget::progress_bar::{Catalog, Style},
    },
};

pub struct ProgressBar;

impl Catalog for Theme {
    type Class<'a> = ProgressBar;

    fn default<'a>() -> Self::Class<'a> {
        ProgressBar
    }

    fn style(&self, _: &Self::Class<'_>) -> Style {
        let palette = self.palette().unwrap();

        Style {
            background: Background::Color(deviate(palette.primary, 0.75)),
            bar: Background::Color(palette.primary),
            border: Border {
                radius: radius(3),
                ..Default::default()
            },
        }
    }
}
