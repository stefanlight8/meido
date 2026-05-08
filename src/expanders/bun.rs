use crate::{
    category::Category,
    expander::{Expanded, Expander},
    node::Node,
    policy::Policy,
};

pub struct BunExpander;

impl Expander<Node> for BunExpander {
    fn expand(&self, item: &Node) -> Expanded<Node> {
        if item.path.ends_with(".bun") {
            return Expanded::Item(item.join("install/cache", Policy::Collect(Category::BunCache)));
        }

        Expanded::None
    }
}
