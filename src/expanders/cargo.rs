use crate::{
    category::Category,
    expander::{Expanded, Expander},
    node::Node,
    policy::Policy,
};

pub struct CargoExpander;

impl Expander<Node> for CargoExpander {
    fn expand(&self, item: &Node) -> Expanded<Node> {
        if item.path.ends_with(".cargo") {
            return Expanded::Vec(vec![
                item.join(
                    "registry/cache",
                    Policy::Collect(Category::CargoRegistryCache),
                ),
                item.join("git", Policy::Collect(Category::CargoGit)),
            ]);
        }

        Expanded::None
    }
}
