pub mod cache;
pub mod disk_images;
pub mod electron;
pub mod gradle;
pub mod maven;
pub mod node;
pub mod python;
pub mod rust;
pub mod vsc;
pub mod zig;

use {crate::category::Category, std::path::Path};

#[async_trait::async_trait]
pub trait Rule: Sync {
    async fn check(&self, path: &Path) -> Option<Category>;
}
