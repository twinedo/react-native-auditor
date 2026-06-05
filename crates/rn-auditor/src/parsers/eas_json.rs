use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct EasJson {
    #[serde(default)]
    build: Option<EasProfiles>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct EasProfiles {
    #[serde(default)]
    production: Option<serde_json::Value>,
}

impl EasJson {
    pub fn read_from(path: &Path) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("Failed to read eas.json: {err}"))?;

        serde_json::from_str::<EasJson>(&content)
            .map_err(|err| format!("Failed to parse eas.json as valid JSON: {err}"))
    }

    pub fn build_production(&self) -> Option<&serde_json::Value> {
        self.build
            .as_ref()
            .and_then(|build| build.production.as_ref())
    }
}
