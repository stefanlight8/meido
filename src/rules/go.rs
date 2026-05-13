use {
    crate::{category::Category, rules::Rule},
    std::path::Path,
};

pub struct GoCacheRule;

#[async_trait::async_trait]
impl Rule for GoCacheRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with("mod/cache") {
            Some(Category::GoCache)
        } else {
            None
        }
    }
}
