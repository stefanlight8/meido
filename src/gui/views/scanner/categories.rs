use {
    crate::gui::{
        element::Element,
        views::scanner::{message::ScannerMessage, state::ScannerState},
    },
    iced::widget::{checkbox, column, mouse_area, row, text},
};

pub fn categories<'a>(state: &'a ScannerState) -> Element<'a, ScannerMessage> {
    let mut tree = column![];
    let categories = state.trash_buffer.categories();

    if categories.size_hint().0 != 0 {
        tree = tree.push(mouse_area(text("All")).on_press(ScannerMessage::View(None)))
    }

    for category in categories {
        tree = tree.push(
            mouse_area(
                row![
                    checkbox(state.selected_categories.contains(&category)).on_toggle(|status| {
                        let category = category.clone();

                        if status {
                            ScannerMessage::SelectCategory(category)
                        } else {
                            ScannerMessage::UnselectCategory(category)
                        }
                    }),
                    text(format!("{:?}", category))
                ]
                .padding(2)
                .spacing(4),
            )
            .on_press(ScannerMessage::View(Some(category.clone()))),
        );
    }

    tree.into()
}
