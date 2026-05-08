use iced::{
    Element, Length,
    widget::{column, scrollable, space, text},
};

use crate::app::{message::Message, state::State};

pub fn category_view(state: &State) -> Element<'_, Message> {
    let category = state.selected_category;

    if let (Some(trash_buffer), Some(category)) = (state.trash_buffer.as_ref(), category) {
        scrollable(column(
            trash_buffer
                .0
                .get(&category)
                .unwrap_or(&vec![])
                .iter()
                .map(|item| text(format!("{:?}", item.display())).into()),
        ))
        .width(Length::Fill)
        .into()
    } else {
        space().into()
    }
}
