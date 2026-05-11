pub mod bun;
pub mod cargo;
pub mod gradle;
pub mod m2;
pub mod npm;
pub mod spotify;

#[cfg(target_os = "macos")]
pub mod library;

pub enum Expanded<T> {
    None,
    Item(T),
    Vec(Vec<T>),
}

pub trait Expander<T>: Sync {
    fn expand(&self, item: &T) -> Expanded<T>;
}
