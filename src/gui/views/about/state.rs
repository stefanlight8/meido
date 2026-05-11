use crate::{
    app::{DESCRIPTION, VERSION},
    gui::{element::Element, message::Message, styles::text::Text, widget::bar},
};
use iced::{
    Length,
    widget::{column, container, space, svg, text},
};

pub struct AboutState;

impl AboutState {
    pub fn view(&self) -> Element<'_, Message> {
        column![
            space().width(Length::Fill).height(Length::Fixed(30.0)),
            column![
                container(svg("assets/logo.svg")).align_left(Length::Fixed(255.0)),
                column![
                    text("Meido").size(24),
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
