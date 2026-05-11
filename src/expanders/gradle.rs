use crate::{
    category::Category,
    expanders::{Expanded, Expander},
    node::Node,
    policy::Policy,
};

pub struct GradleExpander;

impl Expander<Node> for GradleExpander {
    fn expand(&self, item: &Node) -> Expanded<Node> {
        if item.path.ends_with(".gradle") {
            Expanded::Item(item.join("caches", Policy::Collect(Category::GradleCache)))
        } else {
            Expanded::None
        }
    }
}
