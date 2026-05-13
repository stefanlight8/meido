use {
    crate::category::Category,
    std::{
        collections::{HashMap, HashSet},
        path::PathBuf,
    },
};

#[derive(Debug, Clone)]
pub struct TrashEntry(pub Category, pub PathBuf);

#[derive(Debug, Clone)]
pub struct TrashBuffer {
    trash: HashMap<Category, Vec<PathBuf>>,
    pub can: HashSet<PathBuf>,
}

impl TrashBuffer {
    pub fn new() -> Self {
        Self {
            trash: HashMap::new(),
            can: HashSet::new(),
        }
    }

    pub fn categories(&self) -> impl Iterator<Item = &Category> {
        self.trash.keys()
    }

    pub fn files(&self, category: Option<Category>) -> impl Iterator<Item = &PathBuf> + '_ {
        self.trash
            .iter()
            .filter(move |(k, _)| category.as_ref().map_or(true, |c| *k == c))
            .flat_map(|(_, v)| v.iter())
    }

    pub fn push(&mut self, entry: TrashEntry) {
        self.trash.entry(entry.0).or_default().push(entry.1);
    }

    pub fn contains(&self, path: &PathBuf) -> bool {
        self.can.contains(path)
    }

    pub fn trash_category(&mut self, category: &Category) {
        let buffer = self.trash.get(category).cloned().unwrap_or(vec![]);

        self.can.extend(buffer);
    }

    pub fn untrash_category(&mut self, category: &Category) {
        let buffer = self.trash.get(category).cloned().unwrap_or(vec![]);

        buffer.iter().for_each(|k| {
            self.can.remove(k);
        });
    }

    pub async fn trash(self) {}

    pub async fn delete(self) {}
}
