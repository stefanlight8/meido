use {
    crate::{category::Category, rule::Rule},
    std::path::Path,
};

pub struct PdmBuildRule;

#[async_trait::async_trait]
impl Rule for PdmBuildRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with(".pdm-build") {
            return Some(Category::PdmBuilds);
        }

        None
    }
}
