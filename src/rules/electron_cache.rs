use std::path::Path;

use crate::{category::Category, rule::Rule};

pub struct ElectronCacheRule;

#[async_trait::async_trait]
impl Rule for ElectronCacheRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with("DawnCache")
            || path.ends_with("DawnGraphiteCache")
            || path.ends_with("DawnWebGPUCache")
            || path.ends_with("GPUCache")
            || path.ends_with("Code Cache")
            || path.ends_with("Cache_Data")
        {
            Some(Category::ElectronCache)
        } else {
            None
        }
    }
}
