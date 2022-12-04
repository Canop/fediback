use {
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    pub total_items: usize,
    pub ordered_items: Option<Vec<String>>,
    pub first: Option<String>,
    pub next: Option<String>,
}

