use {
    crate::gui::theme::Theme,
    iced::widget::svg::{Catalog, Status, Style},
};

#[derive(Default)]
pub enum Svg {
    #[default]
    Primary,
    Secondary,
}

impl Catalog for Theme {
    type Class<'a> = Svg;

    fn default<'a>() -> Self::Class<'a> {
        Svg::default()
    }

    fn style(&self, _: &Self::Class<'_>, _: Status) -> Style {
        Style::default()
    }
}
