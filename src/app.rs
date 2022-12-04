use {
    crate::*,
    directories_next::ProjectDirs,
};

/// return the instance of ProjectDirs holding the app specific paths
pub fn app_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("org", "dystroy", "fediback")
        .ok_or(Error::Other("Unable to find app directories"))
}
