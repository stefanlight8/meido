use {
    crate::gui::theme::Theme,
    iced::{
        Background, Color,
        theme::Base,
        widget::container::{Catalog, Style},
    },
};

#[derive(Default)]
pub enum Container {
    #[default]
    Default,
    Color(Color),
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
            Container::Color(color) => Style {
                background: Some(Background::Color(color.clone())),
                ..Default::default()
            },
        }
    }
}
