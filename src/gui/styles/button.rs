use {
    crate::gui::theme::Theme,
    iced::{
        Background, Border, Shadow,
        border::radius,
        theme::{
            Base,
            palette::{Danger, Primary, Secondary, Warning},
        },
        widget::button::{Catalog, Status, Style},
    },
};

#[derive(Default)]
pub enum Button {
    #[default]
    Primary,
    Secondary,
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
        let primary = Primary::generate(palette.primary, palette.background, palette.text);
        let secondary = Secondary::generate(palette.background, palette.text);
        let warning = Warning::generate(palette.warning, palette.background, palette.text);
        let danger = Danger::generate(palette.danger, palette.background, palette.text);

        let mut style = Style {
            background: None,
            border: Border {
                radius: radius(12),
                ..Default::default()
            },
            text_color: palette.text,
            shadow: Shadow::default(),
            snap: true,
        };

        match status {
            Status::Active => match class {
                Button::Secondary => {
                    style = style.with_background(Background::Color(secondary.base.color));
                    style.text_color = secondary.base.text;
                }
                _ => {
                    style = style.with_background(Background::Color(primary.base.color));
                    style.text_color = primary.base.text;
                }
            },
            Status::Disabled => match class {
                Button::Secondary => {
                    style = style.with_background(Background::Color(secondary.weak.color));
                    style.text_color = secondary.weak.text;
                }
                _ => {
                    style = style.with_background(Background::Color(primary.weak.color));
                    style.text_color = primary.weak.text;
                }
            },
            Status::Hovered => match class {
                Button::Primary => {
                    style = style.with_background(Background::Color(primary.strong.color));
                    style.text_color = primary.strong.text;
                }
                Button::Secondary => {
                    style = style.with_background(Background::Color(secondary.strong.color));
                    style.text_color = secondary.strong.text;
                }
                Button::Warning => {
                    style = style.with_background(Background::Color(warning.base.color));
                    style.text_color = warning.base.text;
                }
                Button::Danger => {
                    style = style.with_background(Background::Color(danger.base.color));
                    style.text_color = danger.base.text;
                }
            },
            Status::Pressed => match class {
                Button::Primary => {
                    style = style.with_background(Background::Color(primary.weak.color));
                    style.text_color = primary.weak.text;
                }
                Button::Secondary => {
                    style = style.with_background(Background::Color(secondary.weak.color));
                    style.text_color = secondary.weak.text;
                }
                Button::Warning => {
                    style = style.with_background(Background::Color(warning.weak.color));
                    style.text_color = warning.weak.text;
                }
                Button::Danger => {
                    style = style.with_background(Background::Color(danger.weak.color));
                    style.text_color = danger.weak.text;
                }
            },
        }

        style
    }
}
