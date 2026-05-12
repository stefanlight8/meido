use {
    crate::gui::theme::Theme,
    iced::{
        Background, Border,
        border::radius,
        theme::{Base, palette::deviate},
        widget::button::{Catalog, Status, Style},
    },
};

#[derive(Default)]
pub enum Button {
    #[default]
    Primary,
    Warning,
    Danger,
}

impl Catalog for Theme {
    type Class<'a> = Button;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        let palette = self.palette().unwrap();

        Style {
            background: Some(Background::Color(match status {
                Status::Active => palette.primary,
                Status::Disabled => deviate(palette.primary, 0.25),
                Status::Hovered => match class {
                    Button::Primary => deviate(palette.primary, 0.30),
                    Button::Warning => palette.warning,
                    Button::Danger => palette.danger,
                },
                Status::Pressed => deviate(
                    match class {
                        Button::Primary => palette.primary,
                        Button::Warning => palette.warning,
                        Button::Danger => palette.danger,
                    },
                    0.35,
                ),
            })),
            text_color: palette.background,
            border: Border {
                radius: radius(12),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
