use {
    crate::category::Category,
    std::{collections::HashMap, path::PathBuf},
};

#[derive(Debug, Clone)]
pub struct TrashBuffer(pub HashMap<Category, Vec<PathBuf>>);

impl TrashBuffer {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn push(&mut self, category: Category, path: PathBuf) {
        self.0.entry(category).or_default().push(path);
    }

    pub async fn trash(self) {}

    pub async fn delete(self) {}
}
