pub mod bun;
pub mod cargo;
pub mod gradle;
pub mod m2;
pub mod npm;
pub mod spotify;

#[cfg(target_os = "macos")]
pub mod library;
