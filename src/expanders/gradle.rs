use crate::{
    category::Category,
    expander::{Expanded, Expander},
    node::Node,
    policy::Policy,
};

pub struct GradleExpander;

impl Expander<Node> for GradleExpander {
    fn expand(&self, item: &Node) -> crate::expander::Expanded<Node> {
        if item.path.ends_with(".gradle") {
            Expanded::Item(item.join("caches", Policy::Collect(Category::GradleCache)))
        } else {
            Expanded::None
        }
    }
}
