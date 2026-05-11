use {
    crate::{category::Category, rules::Rule},
    std::path::Path,
};

pub struct RustTargetRule;

#[async_trait::async_trait]
impl Rule for RustTargetRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with("target") && path.join(".rustc_info.json").exists() {
            return Some(Category::RustTargets);
        }

        None
    }
}
