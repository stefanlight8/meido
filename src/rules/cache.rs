use {
    crate::{category::Category, rules::Rule},
    std::path::Path,
};

pub struct CacheRule;

#[async_trait::async_trait]
impl Rule for CacheRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with(".cache") {
            Some(Category::Cache)
        } else {
            None
        }
    }
}
