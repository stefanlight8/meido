use {
    crate::{category::Category, rule::Rule},
    std::path::Path,
};

pub struct ZigCacheRule;

#[async_trait::async_trait]
impl Rule for ZigCacheRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with(".zig-cache") {
            Some(Category::ZigCache)
        } else {
            None
        }
    }
}
