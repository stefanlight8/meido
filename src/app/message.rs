use crate::{category::Category, trash_buffer::TrashBuffer};

#[derive(Debug, Clone)]
pub enum Message {
    Scan,
    Scanned(TrashBuffer),
    ScanError(String),
    Trash,
    Delete,
    Trashed(()),
    Deleted(()),
    ToggleCategory(Category, bool),
    OpenCategory(Category),
    Sized(Category, u64),
}
