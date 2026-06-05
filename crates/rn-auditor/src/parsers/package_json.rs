use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PackageJson {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,

    #[serde(default, rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum ProjectType {
    Expo,
    ReactNative,
    Unknown,
}

impl ProjectType {
    pub fn label(&self) -> &'static str {
        match self {
            ProjectType::Expo => "Expo",
            ProjectType::ReactNative => "React Native",
            ProjectType::Unknown => "Unknown",
        }
    }
}

impl PackageJson {
    pub fn read_from(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|err| format!("Failed to read package.json: {err}"))?;

        serde_json::from_str::<PackageJson>(&content)
            .map_err(|err| format!("Failed to parse package.json: {err}"))
    }

    pub fn has_dependency(&self, name: &str) -> bool {
        self.dependencies.contains_key(name) || self.dev_dependencies.contains_key(name)
    }

    pub fn detect_project_type(&self) -> ProjectType {
        if self.has_dependency("expo") {
            ProjectType::Expo
        } else if self.has_dependency("react-native") {
            ProjectType::ReactNative
        } else {
            ProjectType::Unknown
        }
    }
}
