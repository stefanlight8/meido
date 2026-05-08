mod message;
mod sizes;
mod state;
mod utils;
mod views;

use {
    crate::{app::state::State, expander::Expander, node::Node, rule::Rule},
    iced::{Result, window::Settings},
    std::path::PathBuf,
};

pub fn run(
    path: PathBuf,
    expanders: &'static [&'static dyn Expander<Node>],
    rules: &'static [&'static dyn Rule],
) -> Result {
    iced::application(
        move || State::new(path.to_path_buf(), expanders, rules),
        State::update,
        State::view,
    )
    .theme(State::theme)
    .window(Settings::default())
    .title("Meido")
    .run()
}
