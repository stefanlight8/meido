use crate::{
    category::Category,
    expanders::{Expanded, Expander},
    node::Node,
    policy::Policy,
};

pub struct NpmExpander;

impl Expander<Node> for NpmExpander {
    fn expand(&self, item: &Node) -> Expanded<Node> {
        if item.path.ends_with(".npm") {
            Expanded::Item(item.join("_cacache", Policy::Collect(Category::NpmCache)))
        } else {
            Expanded::None
        }
    }
}
