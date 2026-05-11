use {
    crate::gui::palette::{dark::DARK, light::LIGHT},
    iced::theme::{Base, Mode, Palette, Style},
};

pub enum Theme {
    Dark,
    Light,
}

impl Base for Theme {
    fn base(&self) -> Style {
        let palette = self.palette().unwrap_or(DARK);

        Style {
            background_color: palette.background,
            text_color: palette.text,
        }
    }

    fn default(preference: Mode) -> Self {
        match preference {
            Mode::Dark => Self::Dark,
            Mode::Light => Self::Dark,
            _ => Self::Dark,
        }
    }

    fn mode(&self) -> Mode {
        match self {
            Theme::Dark => Mode::Dark,
            Theme::Light => Mode::Light,
        }
    }

    fn name(&self) -> &str {
        match self {
            Theme::Dark => "Meido Dark",
            Theme::Light => "Meido Light",
        }
    }

    fn palette(&self) -> Option<Palette> {
        match self {
            Theme::Dark => Some(DARK),
            Theme::Light => Some(LIGHT),
        }
    }
}
