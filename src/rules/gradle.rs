use crate::{category::Category, rule::Rule};

pub struct GradleBuildRule;

#[async_trait::async_trait]
impl Rule for GradleBuildRule {
    async fn check(&self, path: &std::path::Path) -> Option<crate::category::Category> {
        if path.ends_with("build") && path.join("reports").join("configuration-cache").exists() {
            return Some(Category::GradleBuild);
        }

        None
    }
}

pub struct GradleRule;

#[async_trait::async_trait]
impl Rule for GradleRule {
    async fn check(&self, path: &std::path::Path) -> Option<Category> {
        if path.ends_with(".gradle") {
            return Some(Category::Gradle);
        }

        None
    }
}
