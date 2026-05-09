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
            || path.ends_with(".poetry_cache")
            || path.ends_with(".tox")
            || path.ends_with(".nox")
        {
            return Some(Category::PythonCaches);
        }

        None
    }
}

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
