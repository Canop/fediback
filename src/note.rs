use {
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    pub in_reply_to: Option<String>,
    pub summary: Option<String>,
    pub url: String,
    pub published: String,
    pub to: Option<Vec<String>>,
    pub cc: Option<Vec<String>>,
    pub conversation: Option<String>,
    pub content: String,
}
