use {
    crate::gui::{element::Element, styles::text::Text},
    iced::{
        Length,
        widget::{self, column, row, space, text},
    },
    std::ops::RangeInclusive,
};

pub struct ProgressBar {
    pub range: RangeInclusive<f32>,
    pub value: f32,
    pub girth: f32,
    pub status: Option<String>,
    pub details: Option<String>,
    pub footer: Option<String>,
}

impl ProgressBar {
    pub fn view<'a, Message: 'a>(self) -> Element<'a, Message> {
        let progress_bar = widget::progress_bar(self.range, self.value).girth(self.girth);
        let percentage = text(format!("{:.0}%", self.value * 100.0));
        let mut details_column = column![];

        if let Some(status) = self.status {
            details_column =
                details_column.push(row![text(status), space().width(Length::Fill), percentage]);
        } else {
            details_column = details_column.push(percentage)
        }

        let mut footer_row = row![];
        if let Some(details) = self.details {
            footer_row = footer_row.push(text(details).class(Text::Secondary));
        }
        if let Some(footer) = self.footer {
            footer_row = footer_row.push(space().width(Length::Fill));
            footer_row = footer_row.push(text(footer).class(Text::Secondary));
        }
        details_column = details_column.push(footer_row);

        column![details_column, progress_bar].spacing(12).into()
    }

    pub fn girth(mut self, girth: f32) -> Self {
        self.girth = girth;
        self
    }

    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    pub fn details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    pub fn footer(mut self, footer: String) -> Self {
        self.footer = Some(footer);
        self
    }
}

pub fn progress_bar(range: RangeInclusive<f32>, value: f32) -> ProgressBar {
    ProgressBar {
        range,
        value,
        girth: 32.0,
        status: None,
        details: None,
        footer: None,
    }
}
