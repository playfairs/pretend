use serde::{
  Deserialize,
  Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct App {
  pub name: String,
  pub application_id: String,
  #[serde(default)]
  pub aliases: Vec<String>,
}

impl App {
  pub fn contains_query(&self, query: &str) -> bool {
    let value = format!("{} {}", self.name, self.aliases.join(" "));
    value.to_lowercase().contains(&query.to_lowercase())
  }
}
