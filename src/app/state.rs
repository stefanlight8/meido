use {
    crate::{
        app::{
            message::Message,
            sizes::{Sizes, get_sizes},
            utils::humanize_bytes,
            views::{categories::categories_view, category::category_view},
        },
        category::Category,
        expander::Expander,
        node::Node,
        rule::Rule,
        scan::scan,
        trash_buffer::TrashBuffer,
    },
    iced::{
        Element, Length, Task, Theme,
        widget::{button, column, container, row, space, text},
    },
    std::{collections::HashMap, path::PathBuf},
};

pub struct State {
    pub trash_buffer: Option<TrashBuffer>,
    pub selected_categories: HashMap<Category, bool>,
    pub selected_category: Option<Category>,
    pub sizes: Sizes,
    pub total_size: u64,
    path: PathBuf,
    expanders: &'static [&'static dyn Expander<Node>],
    rules: &'static [&'static dyn Rule],
}

impl State {
    pub fn new(
        path: PathBuf,
        expanders: &'static [&'static dyn Expander<Node>],
        rules: &'static [&'static dyn Rule],
    ) -> Self {
        Self {
            trash_buffer: None,
            selected_categories: HashMap::new(),
            selected_category: None,
            sizes: Sizes::default(),
            total_size: 0,
            path,
            expanders,
            rules,
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::CatppuccinLatte
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Scan => Task::perform(
                scan(self.path.to_path_buf(), self.expanders, self.rules),
                |res| match res {
                    Ok(buffer) => Message::Scanned(buffer),
                    Err(err) => Message::ScanError(err.to_string()),
                },
            ),
            Message::Trash => {
                let buffer = self.trash_buffer.take().unwrap();

                Task::perform(buffer.trash(), Message::Trashed)
            }
            Message::Delete => {
                let buffer = self.trash_buffer.take().unwrap();

                Task::perform(buffer.delete(), Message::Deleted)
            }
            Message::Scanned(trash_buffer) => {
                let task =
                    Task::batch(trash_buffer.clone().0.into_iter().map(|(category, paths)| {
                        Task::perform(
                            async move {
                                let size: u64 = get_sizes(paths).await;
                                (category, size)
                            },
                            |(category, size)| Message::Sized(category, size),
                        )
                    }));
                self.trash_buffer = Some(trash_buffer);

                task
            }
            Message::Deleted(()) => Task::none(),
            Message::Trashed(()) => Task::none(),
            Message::OpenCategory(category) => {
                self.selected_category = Some(category);
                Task::none()
            }
            Message::ToggleCategory(category, value) => {
                self.selected_categories.insert(category, value);
                Task::none()
            }
            Message::ScanError(_) => Task::none(),
            Message::Sized(category, size) => {
                self.sizes.0.insert(category, size);
                self.total_size += size;

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            row![
                container(categories_view(&self))
                    .padding(6)
                    .align_left(Length::FillPortion(1))
                    .height(Length::Fill)
                    .style(container::bordered_box),
                container(category_view(&self))
                    .padding(6)
                    .align_left(Length::FillPortion(2))
                    .height(Length::Fill)
                    .style(container::bordered_box),
            ]
            .spacing(6),
            row![
                button("Scan").on_press_maybe(self.trash_buffer.is_none().then(|| Message::Scan)),
                space().width(Length::Fill),
                text(format!(
                    "Total {} of garbage detected",
                    humanize_bytes(self.total_size)
                ))
                .style(text::secondary),
                space().width(Length::Fill),
                button("Trash")
                    .on_press_maybe(self.trash_buffer.is_some().then(|| Message::Trash))
                    .style(button::warning),
                button("Delete")
                    .on_press_maybe(self.trash_buffer.is_some().then(|| Message::Delete))
                    .style(button::danger)
            ]
            .spacing(6)
        ]
        .spacing(6)
        .padding(6)
        .into()
    }
}
