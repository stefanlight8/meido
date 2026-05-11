use {
    crate::gui::theme::Theme,
    iced::{
        theme::{Base, palette::deviate},
        widget::text::{Catalog, Style},
    },
};

#[derive(Default)]
pub enum Text {
    #[default]
    Primary,
    Secondary,
}

impl Catalog for Theme {
    type Class<'a> = Text;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        let palette = self.palette().unwrap();

        match class {
            Text::Primary => Style::default(),
            Text::Secondary => Style {
                color: Some(deviate(palette.text, 0.25)),
            },
        }
    }
}
