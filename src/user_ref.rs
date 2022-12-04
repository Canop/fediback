use {
    crate::*,
    lazy_regex::*,
    serde::{Deserialize, Serialize},
    std::{
        fmt,
        str::FromStr,
    },
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub struct UserRef {
    pub instance: String,
    pub name: String,
}

impl UserRef {
    pub fn new<N: Into<String>, I: Into<String>>(name: N, instance: I) -> Self {
        Self {
            name: name.into(),
            instance: instance.into(),
        }
    }
    pub fn url(&self) -> String {
        format!("https://{}/users/{}", &self.instance, &self.name)
    }
    pub fn get_user(&self, client: &Client) -> Result<User> {
        let url = self.url();
        let user: User = client.get(&url)?;
        if user.id != url {
            Err(Error::UnconsistentData("response id doesn't match requested user"))
        } else {
            Ok(user)
        }
    }
}

impl FromStr for UserRef {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        // @name@instance or name@instance
        if let Some((_, name, instance)) = regex_captures!(
            r#"^@?([a-z0-9._%+-]+)@([a-z0-9.-]+\.[a-z]{2,})$"#i,
            s,
        ) {
            return Ok(Self::new(name, instance));
        }
        // https://instance/users/name or https://instance/@name
        if let Some((_, instance, name)) = regex_captures!(
            r#"^https://([a-z0-9.-]+\.[a-z]{2,})/(?:users/|@)([a-z0-9._%+-]+)/?$"#i,
            s,
        ) {
            return Ok(Self::new(name, instance));
        }
        Err(Error::InvalidUserRef(s.to_string()))
    }
}

impl fmt::Display for UserRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{}@{}", &self.name, &self.instance)
    }
}

