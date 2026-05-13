use {
    crate::gui::theme::Theme,
    iced::{
        Background, Border, Shadow,
        border::radius,
        theme::Base,
        widget::{
            container,
            scrollable::{AutoScroll, Catalog, Rail, Scroller, Status, Style},
        },
    },
};

pub struct Scrollable;

impl Catalog for Theme {
    type Class<'a> = Scrollable;

    fn default<'a>() -> Self::Class<'a> {
        Scrollable
    }

    fn style(&self, _: &Self::Class<'_>, _: Status) -> Style {
        let palette = self.palette().unwrap();

        let rail = Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                background: Background::Color(palette.primary),
                border: Border {
                    radius: radius(12),
                    width: 3.0,
                    ..Default::default()
                },
            },
        };

        Style {
            container: container::Style::default(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
            auto_scroll: AutoScroll {
                background: Background::Color(palette.background),
                icon: palette.primary,
                border: Border::default(),
                shadow: Shadow::default(),
            },
        }
    }
}
