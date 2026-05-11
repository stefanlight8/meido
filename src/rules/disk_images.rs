use {
    crate::{category::Category, rules::Rule},
    std::path::Path,
};

pub struct DiskImagesRule;

#[async_trait::async_trait]
impl Rule for DiskImagesRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("dmg" | "iso")
        ) {
            return Some(Category::DiskImages);
        }

        None
    }
}
