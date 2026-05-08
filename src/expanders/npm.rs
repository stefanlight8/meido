use crate::{
    category::Category,
    expander::{Expanded, Expander},
    node::Node,
    policy::Policy,
};

pub struct NpmExpander;

impl Expander<Node> for NpmExpander {
    fn expand(&self, item: &Node) -> Expanded<Node> {
        if item.path.ends_with(".npm") {
            return Expanded::Item(item.join("_cacache", Policy::Collect(Category::NpmCache)));
        }

        Expanded::None
    }
}
