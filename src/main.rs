mod app;
mod category;
mod expander;
mod expanders;
mod node;
mod policy;
mod rule;
mod rules;
mod scan;
mod trash_buffer;

use {
    crate::{
        expander::Expander,
        expanders::{
            bun::BunExpander, cargo::CargoExpander, gradle::GradleExpander, m2::M2Expander,
            npm::NpmExpander, spotify::SpotifyExpander,
        },
        node::Node,
        rule::Rule,
        rules::{
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
    std::{env, path::PathBuf},
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

    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map(PathBuf::from)?;

    app::run(home_dir, EXPANDERS, RULES)?;

    Ok(())
}
