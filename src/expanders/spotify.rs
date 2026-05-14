use crate::{
    category::Category,
    expanders::{Expanded, Expander},
    node::Node,
    policy::Policy,
};

pub struct SpotifyExpander;

impl Expander<Node> for SpotifyExpander {
    fn expand(&self, item: &Node) -> Expanded<Node> {
        #[cfg(target_os = "macos")]
        if item.path.ends_with("Library")
            && item
                .path
                .join("Application Support")
                .join("Spotify")
                .exists()
        {
            return Expanded::Item(item.join(
                "Application Support/Spotify/PersistentCache",
                Policy::Collect(Category::SpotifyCache),
            ));
        }

        Expanded::None
    }
}
