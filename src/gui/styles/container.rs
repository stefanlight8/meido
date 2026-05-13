use {
    crate::gui::theme::Theme,
    iced::{
        Background, Border, Color,
        border::radius,
        theme::Base,
        widget::container::{Catalog, Style},
    },
};

#[derive(Default)]
pub enum Container {
    #[default]
    Default,
    Color(Color, bool),
}

impl Catalog for Theme {
    type Class<'a> = Container;

    fn default<'a>() -> Self::Class<'a> {
        Container::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        let palette = self.palette().unwrap();

        match class {
            Container::Default => Style {
                text_color: Some(palette.text),
                ..Default::default()
            },
            Container::Color(color, rounding) => Style {
                background: Some(Background::Color(color.clone())),
                border: Border {
                    radius: rounding.then_some(radius(12)).unwrap_or(radius(0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
