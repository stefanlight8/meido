use {
    super::{
        categories::categories, control_panel::control_panel, file_browser::file_browser,
        message::ScannerMessage,
    },
    crate::{
        category::Category,
        expanders::Expander,
        gui::{element::Element, views::scanner::directory::get_directory, widget::status_bar},
        node::Node,
        policy::Policy,
        rules::Rule,
        scan::{index_nodes, scan_node},
        trash::{TrashBuffer, TrashEntry},
    },
    futures_lite::{StreamExt, stream},
    iced::{
        Length, Task,
        widget::{column, container, row, scrollable},
    },
    std::{collections::HashSet, path::PathBuf},
};

type NodeId = PathBuf;

#[derive(Debug)]
pub enum ScanningState {
    None,
    Scanning {
        nodes_index: HashSet<NodeId>,
        scanned: HashSet<NodeId>,
        path: Option<PathBuf>,
        files_count: usize,
    },
    Complete {
        files_count: usize,
    },
}

pub enum ViewFilesState {
    All,
    Category(Category),
}

pub struct ScannerState {
    pub trash_buffer: TrashBuffer,
    pub expanders: &'static [&'static dyn Expander<Node>],
    pub rules: &'static [&'static dyn Rule],
    pub scanning: ScanningState,
    pub selected_categories: HashSet<Category>,
    pub view_files: ViewFilesState,
}

impl ScannerState {
    pub fn new(
        expanders: &'static [&'static dyn Expander<Node>],
        rules: &'static [&'static dyn Rule],
    ) -> Self {
        Self {
            expanders,
            rules,
            trash_buffer: TrashBuffer::new(),
            scanning: ScanningState::None,
            selected_categories: HashSet::new(),
            view_files: ViewFilesState::All,
        }
    }

    pub fn update(&mut self, message: ScannerMessage) -> Task<ScannerMessage> {
        match message {
            ScannerMessage::Scan => Task::perform(get_directory(), ScannerMessage::SelectDirectory),
            ScannerMessage::IndexedNodes(nodes) => {
                self.scanning = ScanningState::Scanning {
                    nodes_index: nodes.iter().map(|node| node.path.clone()).collect(),
                    scanned: HashSet::new(),
                    path: None,
                    files_count: 0,
                };

                Task::batch(nodes.iter().map(|node| match node.policy {
                    Policy::Collect(category) => {
                        match &mut self.scanning {
                            ScanningState::Scanning { scanned, .. } => {
                                scanned.insert(node.path.clone());
                            }
                            _ => (),
                        };

                        Task::done(ScannerMessage::Collected(TrashEntry(
                            category,
                            node.path.clone(),
                        )))
                    }
                    Policy::Scan => {
                        let node = node.clone();

                        Task::stream(
                            scan_node(node.clone(), self.rules)
                                .map(move |entry| ScannerMessage::Collected(entry))
                                .chain(stream::once(ScannerMessage::NodeFinished(node))),
                        )
                    }
                }))
            }
            ScannerMessage::Collected(entry) => {
                match &mut self.scanning {
                    ScanningState::Scanning {
                        path, files_count, ..
                    } => {
                        *path = Some(entry.1.clone());
                        *files_count += 1;
                    }
                    _ => (),
                };
                self.trash_buffer.push(entry);

                Task::none()
            }
            ScannerMessage::NodeFinished(node) => {
                match &mut self.scanning {
                    ScanningState::Scanning {
                        nodes_index,
                        scanned,
                        files_count,
                        ..
                    } => {
                        scanned.insert(node.path.clone());

                        if scanned == nodes_index {
                            self.scanning = ScanningState::Complete {
                                files_count: files_count.clone(),
                            };
                        }
                    }
                    _ => (),
                };
                Task::none()
            }
            ScannerMessage::SelectCategory(category) => {
                self.trash_buffer.trash_category(&category);
                self.selected_categories.insert(category);

                Task::none()
            }
            ScannerMessage::UnselectCategory(category) => {
                self.trash_buffer.untrash_category(&category);
                self.selected_categories.remove(&category);

                Task::none()
            }
            ScannerMessage::View(Some(category)) => {
                self.view_files = ViewFilesState::Category(category);

                Task::none()
            }
            ScannerMessage::View(None) => {
                self.view_files = ViewFilesState::All;

                Task::none()
            }
            ScannerMessage::SelectDirectory(Some(path)) => Task::perform(
                index_nodes(path, self.expanders),
                |res| ScannerMessage::IndexedNodes(res.unwrap_or(vec![])), // TEMP
            ),
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, ScannerMessage> {
        let files = match &self.view_files {
            ViewFilesState::All => self.trash_buffer.files(None),
            ViewFilesState::Category(category) => self.trash_buffer.files(Some(*category)),
        };

        column![
            status_bar(match self.scanning {
                ScanningState::None => "",
                ScanningState::Scanning { .. } => "Scanning",
                ScanningState::Complete { .. } => "",
            }),
            column![
                row![
                    container(categories(&self)).width(Length::FillPortion(1)),
                    container(scrollable(file_browser(&self, files)).width(Length::Fill))
                        .width(Length::FillPortion(3))
                ]
                .height(Length::Fill),
                control_panel(&self)
            ]
            .padding(12),
        ]
        .into()
    }
}
