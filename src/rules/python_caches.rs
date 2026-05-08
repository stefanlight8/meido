use {
    crate::{category::Category, rule::Rule},
    std::path::Path,
};

pub struct PythonCachesRule;

#[async_trait::async_trait]
impl Rule for PythonCachesRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with("__pycache__")
            || path.ends_with(".pytest_cache")
            || path.ends_with(".mypy_cache")
            || path.ends_with(".ruff_cache")
            || path.ends_with(".ty_cache")
            || path.ends_with(".poetry_cache")
            || path.ends_with(".tox")
            || path.ends_with(".nox")
        {
            return Some(Category::PythonCaches);
        }

        None
    }
}
