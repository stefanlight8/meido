use crate::category::Category;
use crate::expander::{Expanded, Expander};
use crate::node::Node;
use crate::policy::Policy;

pub struct LibraryExpander;

impl Expander<Node> for LibraryExpander {
    fn expand(&self, item: &Node) -> Expanded<Node> {
        if item.path.ends_with("Library") {
            return Expanded::Vec(vec![
                item.join("Caches", Policy::Collect(Category::Cache)),
                item.join(
                    "Developer/Xcode/DerivedData",
                    Policy::Collect(Category::Xcode),
                ),
                item.join("Developer/Xcode/Archives", Policy::Collect(Category::Xcode)),
                item.join(
                    "Developer/Xcode/iOS DeviceSupport",
                    Policy::Collect(Category::Xcode),
                ),
                item.join("Developer/CoreSimulator", Policy::Collect(Category::Xcode)),
                item.join("Logs", Policy::Collect(Category::Logs)),
            ]);
        }

        Expanded::None
    }
}
