#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub enum Category {
    Cache,
    Logs,
    RustTargets,
    NodeModules,
    PythonVenvs,
    PythonCaches,
    BunCache,
    GoCache,
    CargoGit,
    CargoRegistryCache,
    DiskImages,
    GradleBuild,
    Gradle,
    MavenTarget,
    M2Repository,
    PdmBuilds,
    NpmCache,
    #[cfg(target_os = "macos")]
    Xcode,
}
