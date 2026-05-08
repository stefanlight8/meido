use crate::{
    app::{message::Message, state::State, utils::humanize_bytes},
    category::Category,
};
use iced::{
    Alignment, Element, Length,
    widget::{button, checkbox, column, container, mouse_area, row, scrollable, space, text},
};

fn category_view(category: &Category, selected: bool, size: Option<u64>) -> Element<'_, Message> {
    let category = category.clone();
    let checkbox = checkbox(selected).on_toggle(move |v| Message::ToggleCategory(category, v));
    let label = text(format!("{:?}", category));
    let size = size
        .map_or(text("Loading"), |size| text(humanize_bytes(size)))
        .style(text::secondary);
    let content = row![checkbox, label, space().width(Length::Fill), size]
        .spacing(6)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    mouse_area(container(content).padding(6).width(Length::Fill))
        .on_press(Message::OpenCategory(category))
        .into()
}

pub fn categories_view(state: &State) -> Element<'_, Message> {
    if let Some(trash_buffer) = state.trash_buffer.as_ref() {
        scrollable(column![
            row![button("Select all"), button("Unselect all")],
            column(trash_buffer.0.iter().map(|(category, _)| {
                category_view(
                    category,
                    state
                        .selected_categories
                        .get(&category)
                        .copied()
                        .unwrap_or(false),
                    state.sizes.0.get(category).copied(),
                )
            }))
            .spacing(6)
        ])
        .width(Length::Fill)
        .spacing(6)
        .into()
    } else {
        container("No categories").padding(6).into()
    }
}
