use {
    crate::category::Category,
    std::{collections::HashMap, path::PathBuf},
};

#[derive(Debug, Clone)]
pub struct TrashEntry(pub Category, pub PathBuf);

#[derive(Debug, Clone)]
pub struct TrashBuffer(pub HashMap<Category, Vec<PathBuf>>);

impl TrashBuffer {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn push(&mut self, entry: TrashEntry) {
        self.0.entry(entry.0).or_default().push(entry.1);
    }

    pub async fn trash(self) {}

    pub async fn delete(self) {}
}
