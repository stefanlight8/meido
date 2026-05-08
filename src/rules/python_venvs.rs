use {
    crate::{category::Category, rule::Rule},
    std::path::Path,
};

pub struct PythonVenvsRule;

#[async_trait::async_trait]
impl Rule for PythonVenvsRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with(".venv") {
            return Some(Category::PythonVenvs);
        }

        None
    }
}
