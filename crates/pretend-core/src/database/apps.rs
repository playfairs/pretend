use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::database::search::SearchResult;
use crate::error::{Error, Result};
use crate::models::App;
use crate::util::paths;

#[derive(Debug)]
pub struct AppDatabase {
    apps: Vec<App>,
    source: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ApplicationsFile {
    applications: Vec<App>,
}

impl AppDatabase {
    pub fn load() -> Result<Self> {
        let source = paths::applications_path()?;
        let apps = load_apps_from_path(&source)?;
        Ok(Self { apps, source })
    }

    pub fn all(&self) -> &[App] {
        &self.apps
    }

    pub fn find(&self, query: &str) -> Result<App> {
        let normalized = query.trim().to_lowercase();
        if normalized.is_empty() {
            return Err(Error::ApplicationNotFound("empty query".into()));
        }

        self.apps
            .iter()
            .find(|app| app.name.to_lowercase() == normalized || app.aliases.iter().any(|alias| alias.to_lowercase() == normalized))
            .cloned()
            .or_else(|| {
                self.apps
                    .iter()
                    .find(|app| app.contains_query(&normalized))
                    .cloned()
            })
            .ok_or_else(|| Error::ApplicationNotFound(query.into()))
    }

    pub fn search(&self, query: &str) -> Vec<App> {
        let normalized = query.trim().to_lowercase();
        if normalized.is_empty() {
            return self.apps.clone();
        }

        let mut results: Vec<SearchResult> = self
            .apps
            .iter()
            .filter_map(|app| {
                let score = app_score(&normalized, app);
                if score > 0 {
                    Some(SearchResult::new(app.clone(), score))
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|left, right| right.score.cmp(&left.score));
        results.into_iter().map(|entry| entry.app).collect()
    }
}

fn load_apps_from_path(path: &Path) -> Result<Vec<App>> {
    let content = fs::read_to_string(path)?;
    let file: ApplicationsFile = serde_json::from_str(&content)?;
    Ok(file.applications)
}

fn app_score(query: &str, app: &App) -> i32 {
    let name = app.name.to_lowercase();
    let aliases = app.aliases.join(" ").to_lowercase();
    let haystack = format\!("{} {}", name, aliases);

    if haystack.contains(query) {
        return 100;
    }

    if name.contains(query) || aliases.contains(query) {
        return 50;
    }

    let mut score = 0;
    for (index, query_char) in query.chars().enumerate() {
        if let Some(candidate) = haystack.chars().nth(index) {
            if candidate == query_char {
                score += 1;
            }
        }
    }
    score
}
