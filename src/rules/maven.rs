use {
    crate::{category::Category, rules::Rule},
    std::path::Path,
    tokio::fs::read_dir,
};

pub struct MavenRule;

#[async_trait::async_trait]
impl Rule for MavenRule {
    async fn check(&self, path: &Path) -> Option<Category> {
        if path.ends_with("target") {
            if let Ok(mut dir) = read_dir(path).await {
                while let Ok(Some(entry)) = dir.next_entry().await {
                    if entry.path().extension().and_then(|s| s.to_str()) == Some("jar") {
                        return Some(Category::MavenTarget);
                    }
                }
            }
        }

        None
    }
}
