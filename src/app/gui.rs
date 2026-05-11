use {
    crate::{expanders::Expander, gui::state::State, node::Node, rules::Rule},
    anyhow::Result,
    iced::{Font, window},
};

#[cfg(target_os = "macos")]
fn default_font() -> iced::Font {
    Font::with_name("SF Pro")
}

#[cfg(not(target_os = "macos"))]
fn default_font() -> iced::Font {
    Font::with_name("Inter")
}

pub fn run(
    expanders: &'static [&'static dyn Expander<Node>],
    rules: &'static [&'static dyn Rule],
) -> Result<()> {
    iced::application(
        move || State::new(expanders, rules),
        State::update,
        State::view,
    )
    .theme(State::theme)
    .window(window::Settings {
        icon: window::icon::from_file("assets/icon.png")
            .map(Some)
            .unwrap_or(None),
        platform_specific: {
            #[cfg(target_os = "macos")]
            iced::window::settings::PlatformSpecific {
                titlebar_transparent: true,
                fullsize_content_view: true,
                ..Default::default()
            }
        },
        ..Default::default()
    })
    .settings(iced::Settings {
        default_font: default_font(),
        ..Default::default()
    })
    .title("Meido")
    .run()?;

    Ok(())
}
