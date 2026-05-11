use crate::{
    category::Category,
    expanders::{Expanded, Expander},
    node::Node,
    policy::Policy,
};

pub struct BunExpander;

impl Expander<Node> for BunExpander {
    fn expand(&self, item: &Node) -> Expanded<Node> {
        if item.path.ends_with(".bun") {
            Expanded::Item(item.join("install/cache", Policy::Collect(Category::BunCache)))
        } else {
            Expanded::None
        }
    }
}
