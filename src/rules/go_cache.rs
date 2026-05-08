use std::path::Path;

use crate::{category::Category, rule::Rule};

pub struct GoCacheRule;

#[async_trait::async_trait]
impl Rule for GoCacheRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        None
    }
}
