use {
    crate::{category::Category, node::Node, trash::TrashEntry},
    std::path::PathBuf,
};

#[derive(Debug, Clone)]
pub enum ScannerMessage {
    Scan,
    IndexedNodes(Vec<Node>),
    NodeFinished(Node),
    Collected(TrashEntry),
    SelectCategory(Category),
    UnselectCategory(Category),
    SelectFile(PathBuf),
    UnselectFile(PathBuf),
    View(Option<Category>),
    SelectedCategory(Option<PathBuf>),
    SelectDirectory(Option<PathBuf>),
    Error(String),
}
