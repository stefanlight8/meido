use {
    super::message::ScannerMessage,
    crate::{
        expanders::Expander,
        gui::{element::Element, styles::button::Button, widget::progress_bar},
        node::Node,
        policy::Policy,
        rules::Rule,
        scan::{index_nodes, scan_node},
        trash::{TrashBuffer, TrashEntry},
    },
    futures_lite::{StreamExt, stream},
    iced::{
        Length, Task,
        widget::{button, column, row, space},
    },
    std::{collections::HashSet, env, path::PathBuf},
};

type NodeId = PathBuf;

#[derive(Debug)]
enum ScanningState {
    None,
    Scanning {
        nodes_index: HashSet<NodeId>,
        scanned: HashSet<NodeId>,
        path: Option<PathBuf>,
    },
    Complete,
}

pub struct ScannerState {
    pub path: PathBuf,
    pub trash_buffer: TrashBuffer,
    pub expanders: &'static [&'static dyn Expander<Node>],
    pub rules: &'static [&'static dyn Rule],
    scanning: ScanningState,
}

impl ScannerState {
    pub fn new(
        expanders: &'static [&'static dyn Expander<Node>],
        rules: &'static [&'static dyn Rule],
    ) -> Self {
        Self {
            expanders,
            rules,
            path: env::var("HOME")
                .or_else(|_| env::var("USERPROFILE"))
                .map_or(PathBuf::from("~"), PathBuf::from), // TODO: path settings
            trash_buffer: TrashBuffer::new(),
            scanning: ScanningState::None,
        }
    }

    pub fn update(&mut self, message: ScannerMessage) -> Task<ScannerMessage> {
        match message {
            ScannerMessage::Scan(path) => Task::perform(
                index_nodes(path, self.expanders),
                |res| ScannerMessage::IndexedNodes(res.unwrap_or(vec![])), // TEMP
            ),
            ScannerMessage::IndexedNodes(nodes) => {
                self.scanning = ScanningState::Scanning {
                    nodes_index: nodes.iter().map(|node| node.path.clone()).collect(),
                    scanned: HashSet::new(),
                    path: None,
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
                                .chain(stream::once(ScannerMessage::NodeFinished(node.clone()))),
                        )
                    }
                }))
            }
            ScannerMessage::Collected(entry) => {
                match &mut self.scanning {
                    ScanningState::Scanning { path, .. } => {
                        *path = Some(entry.1.clone());
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
                        ..
                    } => {
                        scanned.insert(node.path.clone());

                        if scanned == nodes_index {
                            self.scanning = ScanningState::Complete;
                        }
                    }
                    _ => (),
                };
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, ScannerMessage> {
        let content: Element<'_, ScannerMessage> = match &self.scanning {
            ScanningState::None => button("Scan")
                .on_press(ScannerMessage::Scan(self.path.clone()))
                .into(),

            ScanningState::Scanning {
                nodes_index,
                scanned,
                ..
            } => progress_bar(0.0..=1.0, scanned.len() as f32 / nodes_index.len() as f32)
                .girth(3.0)
                .status("Scanning".to_string())
                .details(format!("{}/{}", scanned.len(), nodes_index.len()))
                .view()
                .into(),

            ScanningState::Complete => row![
                button("Trash").class(Button::Warning),
                button("Delete").class(Button::Danger),
            ]
            .spacing(4)
            .into(),
        };

        column![space().height(Length::Fill), content]
            .padding(12)
            .into()
    }
}
