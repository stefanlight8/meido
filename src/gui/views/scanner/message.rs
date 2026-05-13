use {
    crate::{category::Category, node::Node, trash::TrashEntry},
    std::path::PathBuf,
};

#[derive(Debug, Clone)]
pub enum ScannerMessage {
    Scan(PathBuf),
    IndexedNodes(Vec<Node>),
    NodeFinished(Node),
    Collected(TrashEntry),
    SelectCategory(Category),
    UnselectCategory(Category),
    SelectFile(PathBuf),
    UnselectFile(PathBuf),
    View(Option<Category>),
}
