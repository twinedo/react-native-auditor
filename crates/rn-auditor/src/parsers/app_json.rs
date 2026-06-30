use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AppJson {
    #[serde(default)]
    pub expo: Option<ExpoConfig>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ExpoConfig {
    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub slug: Option<String>,

    #[serde(default)]
    pub version: Option<String>,

    #[serde(default)]
    pub ios: Option<IosConfig>,

    #[serde(default)]
    pub android: Option<AndroidConfig>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct IosConfig {
    #[serde(default, rename = "bundleIdentifier")]
    pub bundle_identifier: Option<String>,

    #[serde(default, rename = "buildNumber")]
    pub build_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AndroidConfig {
    #[serde(default, rename = "package")]
    pub package_name: Option<String>,

    #[serde(default, rename = "versionCode")]
    pub version_code: Option<i64>,
}

impl AppJson {
    pub fn read_from(path: &Path) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("Failed to read app.json: {err}"))?;

        serde_json::from_str::<AppJson>(&content)
            .map_err(|err| format!("Failed to parse app.json as valid JSON: {err}"))
    }

    pub fn ios_bundle_identifier(&self) -> Option<&str> {
        self.expo
            .as_ref()
            .and_then(|expo| expo.ios.as_ref())
            .and_then(|ios| ios.bundle_identifier.as_deref())
    }

    pub fn name(&self) -> Option<&str> {
        self.expo.as_ref().and_then(|expo| expo.name.as_deref())
    }

    pub fn slug(&self) -> Option<&str> {
        self.expo.as_ref().and_then(|expo| expo.slug.as_deref())
    }

    pub fn version(&self) -> Option<&str> {
        self.expo.as_ref().and_then(|expo| expo.version.as_deref())
    }

    pub fn ios_build_number(&self) -> Option<&str> {
        self.expo
            .as_ref()
            .and_then(|expo| expo.ios.as_ref())
            .and_then(|ios| ios.build_number.as_deref())
    }

    pub fn android_package(&self) -> Option<&str> {
        self.expo
            .as_ref()
            .and_then(|expo| expo.android.as_ref())
            .and_then(|android| android.package_name.as_deref())
    }

    pub fn android_version_code(&self) -> Option<i64> {
        self.expo
            .as_ref()
            .and_then(|expo| expo.android.as_ref())
            .and_then(|android| android.version_code)
    }
}
