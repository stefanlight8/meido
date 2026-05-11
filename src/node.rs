use {
    crate::policy::Policy,
    std::path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Node {
    pub path: PathBuf,
    pub policy: Policy,
}

impl Node {
    pub fn new(path: PathBuf, policy: Policy) -> Node {
        Node { path, policy }
    }

    pub fn join(&self, path: impl AsRef<Path>, policy: Policy) -> Node {
        Node::new(self.path.join(path), policy)
    }
}
