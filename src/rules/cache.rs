use std::path::Path;

use crate::{category::Category, rule::Rule};

pub struct CacheRule;

#[async_trait::async_trait]
impl Rule for CacheRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with(".cache") {
            return Some(Category::Cache);
        }

        None
    }
}
