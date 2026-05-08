use crate::{
    category::Category,
    expander::{Expanded, Expander},
    node::Node,
    policy::Policy,
};

pub struct M2Expander;

impl Expander<Node> for M2Expander {
    fn expand(&self, item: &Node) -> Expanded<Node> {
        if item.path.ends_with(".m2") {
            return Expanded::Item(
                item.join("repository", Policy::Collect(Category::M2Repository)),
            );
        }

        Expanded::None
    }
}
