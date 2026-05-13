use {
    crate::{
        app::{DESCRIPTION, VERSION},
        gui::{
            element::Element,
            fonts::inter::INTER_MEDIUM,
            styles::text::Text,
            views::about::message::AboutMessage,
            widget::{bar, status_bar},
        },
    },
    iced::{
        Length,
        widget::{column, container, svg, text},
    },
};

pub struct AboutState;

impl AboutState {
    pub fn view(&self) -> Element<'_, AboutMessage> {
        column![
            status_bar("About"),
            column![
                container(svg("static/logo.svg")).align_left(Length::Fixed(255.0)),
                column![
                    text("Meido").size(20).font(INTER_MEDIUM),
                    text(format!("v{}", VERSION))
                        .class(Text::Secondary)
                        .size(16)
                ]
                .spacing(4),
                text(format!("{}", DESCRIPTION))
                    .class(Text::Secondary)
                    .size(16),
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16)
            .spacing(16),
            bar(Length::Fixed(6.0)).width(Length::Fill)
        ]
        .into()
    }
}
