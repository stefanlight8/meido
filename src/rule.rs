use std::path::Path;

use async_trait::async_trait;

use crate::category::Category;

#[async_trait]
pub trait Rule: Sync {
    async fn check(&self, path: &Path) -> Option<Category>;
}
