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
            background: Some(Background::Color(match (class, status) {
                (Button::Primary, Status::Active) => palette.primary,
                (Button::Primary, Status::Hovered) => deviate(palette.primary, 0.25),
                (Button::Primary, Status::Pressed) => deviate(palette.primary, 0.35),
                (Button::Primary, Status::Disabled) => deviate(palette.primary, 0.45),
                (Button::Warning, Status::Active) => palette.primary,
                (Button::Warning, Status::Hovered) => palette.warning,
                (Button::Warning, Status::Pressed) => deviate(palette.warning, 0.25),
                (Button::Warning, Status::Disabled) => deviate(palette.primary, 0.35),
                (Button::Danger, Status::Active) => palette.primary,
                (Button::Danger, Status::Hovered) => palette.danger,
                (Button::Danger, Status::Pressed) => deviate(palette.danger, 0.25),
                (Button::Danger, Status::Disabled) => deviate(palette.primary, 0.35),
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
