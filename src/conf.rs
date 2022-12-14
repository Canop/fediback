use {
    crate::*,
    serde::{Deserialize, Serialize},
    std::{
        collections::HashSet,
        fs,
        io::Write,
        path::{Path, PathBuf},
    },
    termimad::{MadSkin, mad_print_inline},
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Conf {
    pub watched: HashSet<UserRef>,
}

impl Conf {
    /// Return the standard path to the configuration file
    pub fn standard_location() -> Result<PathBuf> {
        app_dirs().map(|dirs| dirs.config_dir().join("config.json"))
    }
    /// read the configuration (return the default if file isn't found)
    pub fn read(path: &Path) -> Result<Self> {
        if path.exists() {
            let file_content = fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&file_content)?)
        } else {
            Ok(Self::default())
        }
    }
    /// write the conf at the given location
    pub fn save(&self, path: &Path, skin: &MadSkin) -> Result<()> {
        fs::create_dir_all(path.parent().unwrap())?;
        let mut file = fs::File::create(&path)?;
        let json = serde_json::to_string_pretty(self)?;
        write!(&mut file, "{}", json)?;
        mad_print_inline!(
            skin,
            "Configuration saved in *$0*\n",
            path.to_string_lossy(),
        );
        Ok(())
    }
    pub fn add(&mut self, user: UserRef) {
        self.watched.insert(user);
    }
    pub fn remove(&mut self, user: &UserRef) {
        self.watched.remove(user);
    }
}
