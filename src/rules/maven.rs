use tokio::fs::read_dir;

use crate::rule::Rule;

pub struct MavenRule;

#[async_trait::async_trait]
impl Rule for MavenRule {
    async fn check(&self, path: &std::path::Path) -> Option<crate::category::Category> {
        if path.ends_with("target") {
            if let Ok(mut dir) = read_dir(path).await {
                while let Ok(Some(entry)) = dir.next_entry().await {
                    if entry.path().extension().and_then(|s| s.to_str()) == Some("jar") {
                        return Some(crate::category::Category::MavenTarget);
                    }
                }
            }
        }

        None
    }
}
