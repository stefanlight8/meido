use {
    crate::{category::Category, rule::Rule},
    std::path::Path,
};

pub struct ZigBuildRule;

#[async_trait::async_trait]
impl Rule for ZigBuildRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with("zig-out") && path.join("bin").exists() {
            Some(Category::ZigBuild)
        } else {
            None
        }
    }
}
