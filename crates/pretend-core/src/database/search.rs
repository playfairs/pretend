use crate::models::App;

#[derive(Debug, Clone)]
pub struct SearchResult {
  pub app: App,
  pub score: i32,
}

impl SearchResult {
  pub fn new(app: App, score: i32) -> Self {
    Self { app, score }
  }
}
