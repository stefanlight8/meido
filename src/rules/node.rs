use {
    crate::{category::Category, rule::Rule},
    std::path::Path,
};

pub struct NodeModulesRule;

#[async_trait::async_trait]
impl Rule for NodeModulesRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with("node_modules") {
            return Some(Category::NodeModules);
        }

        None
    }
}
