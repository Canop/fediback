use {
    crate::*,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub url: String,
    pub featured: Option<String>,
    pub following: Option<String>,
    pub followers: Option<String>,
    pub public_key: PublicKey,
}
