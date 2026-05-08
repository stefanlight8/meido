use crate::{
    category::Category,
    expander::{Expanded, Expander},
    node::Node,
    policy::Policy,
};

pub struct GoExpander;

impl Expander<Node> for GoExpander {
    fn expand(&self, item: &Node) -> Expanded<Node> {
        if item.path.ends_with(".go") {
            return Expanded::Item(item.join("pkg/mod/cache", Policy::Collect(Category::GoCache)));
        }

        Expanded::None
    }
}
