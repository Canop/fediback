use {
    crate::*,
    serde::{Deserialize, Serialize},
    std::{
        collections::HashSet,
        fs,
        io::Write,
        path::PathBuf,
    },
    termimad::{MadSkin, mad_print_inline},
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Conf {
    pub watched: HashSet<UserRef>,
}

impl Conf {
    pub fn path() -> Result<PathBuf> {
        app_dirs().map(|dirs| dirs.config_dir().join("config.json"))
    }
    /// read the configuration from its standard location
    /// or return the default
    pub fn read() -> Result<Self> {
        let path = Self::path()?;
        if path.exists() {
            let file_content = fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&file_content)?)
        } else {
            Ok(Self::default())
        }
    }
    /// write the conf at its standard location
    pub fn save(&self, skin: &MadSkin) -> Result<()> {
        let path = Self::path()?;
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
