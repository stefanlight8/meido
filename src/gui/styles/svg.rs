use {
    crate::gui::theme::Theme,
    iced::{
        theme::Base,
        widget::svg::{Catalog, Status, Style},
    },
};

pub struct Svg;

impl Catalog for Theme {
    type Class<'a> = Svg;

    fn default<'a>() -> Self::Class<'a> {
        Svg
    }

    fn style(&self, _: &Self::Class<'_>, _: Status) -> Style {
        let palette = self.palette().unwrap();

        Style {
            color: Some(palette.text),
        }
    }
}
