mod app;
mod category;
mod expanders;
mod gui;
mod node;
mod policy;
mod rules;
mod scan;
mod trash;

use {
    crate::{
        expanders::{
            Expander, bun::BunExpander, cargo::CargoExpander, gradle::GradleExpander,
            m2::M2Expander, npm::NpmExpander, spotify::SpotifyExpander,
        },
        node::Node,
        rules::{
            Rule,
            cache::CacheRule,
            disk_images::DiskImagesRule,
            electron::ElectronCacheRule,
            gradle::{GradleBuildRule, GradleRule},
            maven::MavenRule,
            node::NodeModulesRule,
            python::{PdmBuildRule, PythonCachesRule, PythonVenvsRule},
            rust::RustTargetRule,
            vsc::VscCacheRule,
            zig::{ZigBuildRule, ZigCacheRule},
        },
    },
    anyhow::Result,
    tracing::level_filters::LevelFilter,
};

#[cfg(target_os = "macos")]
use crate::expanders::library::LibraryExpander;

static EXPANDERS: &[&dyn Expander<Node>] = &[
    &BunExpander,
    &CargoExpander,
    &M2Expander,
    &NpmExpander,
    &GradleExpander,
    &SpotifyExpander,
    #[cfg(target_os = "macos")]
    &LibraryExpander,
];
static RULES: &[&dyn Rule] = &[
    &CacheRule,
    &NodeModulesRule,
    &RustTargetRule,
    &PythonVenvsRule,
    &PythonCachesRule,
    &DiskImagesRule,
    &GradleRule,
    &GradleBuildRule,
    &MavenRule,
    &PdmBuildRule,
    &ElectronCacheRule,
    &VscCacheRule,
    &ZigBuildRule,
    &ZigCacheRule,
];

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    app::gui::run(EXPANDERS, RULES)?;

    Ok(())
}
