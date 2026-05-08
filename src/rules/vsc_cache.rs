use std::path::Path;

use crate::{category::Category, rule::Rule};

pub struct VscCacheRule;

#[async_trait::async_trait]
impl Rule for VscCacheRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with("CachedConfigurations")
            || path.ends_with("CachedData")
            || path.ends_with("CachedExtensionVSIXs")
            || path.ends_with("CachedProfilesData")
        {
            Some(Category::VscCache)
        } else {
            None
        }
    }
}
