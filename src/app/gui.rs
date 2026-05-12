use {
    crate::{
        expanders::Expander,
        gui::{fonts::inter::INTER_REGULAR, state::State},
        node::Node,
        rules::Rule,
    },
    anyhow::Result,
    iced::window,
};

#[cfg(target_os = "macos")]
fn platform_specific() -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific {
        titlebar_transparent: true,
        fullsize_content_view: true,
        ..Default::default()
    }
}

#[cfg(not(target_os = "macos"))]
fn platform_specific() -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific::default()
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
        platform_specific: platform_specific(),
        ..Default::default()
    })
    .settings(iced::Settings {
        default_font: INTER_REGULAR,
        ..Default::default()
    })
    .title("Meido")
    .run()?;

    Ok(())
}
