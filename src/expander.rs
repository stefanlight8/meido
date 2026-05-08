pub enum Expanded<T> {
    None,
    Item(T),
    Vec(Vec<T>),
}

pub trait Expander<T>: Sync {
    fn expand(&self, item: &T) -> Expanded<T>;
}
