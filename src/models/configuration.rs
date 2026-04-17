use anyhow::Result;
use directories::UserDirs;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{ops::RangeInclusive, path::PathBuf};

use crate::models::image_meta::Color;

use super::ScoreFilter;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(rename = "root_path")]
    pub root_images_dir: PathBuf,

    #[serde(rename = "meta_path")]
    pub metadata_path: PathBuf,

    #[serde(rename = "filters")]
    pub filters: ConfigurationFilters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationFilters {
    #[serde(rename = "active_dirs")]
    pub active_directories: Option<Vec<PathBuf>>,

    #[serde(rename = "scores")]
    pub scores: Option<Vec<ScoreFilter>>,

    #[serde(rename = "width")]
    pub width_range: Option<RangeInclusive<usize>>,

    #[serde(rename = "height")]
    pub height_range: Option<RangeInclusive<usize>>,

    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,

    #[serde(rename = "colors")]
    pub colors: Option<Vec<Color>>,
}

impl Configuration {
    pub fn create_default() -> Configuration {
        let mut root_images_dir = PathBuf::new();
        let mut metadata_path = PathBuf::new();

        if let Some(user_dirs) = UserDirs::new() {
            info!("found user dirs: {:?}", user_dirs);
            if let Some(picture_dir) = user_dirs.picture_dir() {
                info!("found user pictures dir: {}", picture_dir.display());
                root_images_dir = picture_dir.to_path_buf();
                metadata_path = picture_dir.join("metadatas.json");
            }
        }

        let filters = ConfigurationFilters {
            active_directories: None,
            scores: None,
            width_range: Some(RangeInclusive::new(0, 10_000)),
            height_range: Some(RangeInclusive::new(0, 10_000)),
            tags: None,
            colors: None,
        };

        Configuration {
            root_images_dir,
            metadata_path,
            filters,
        }
    }

    pub fn to_toml_str(&self) -> Result<String> {
        let toml = toml::to_string(&self)?;
        debug!("config serialized to TOML: {}", toml);
        Ok(toml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use toml;

    #[test]
    fn test_roundtrip_serialization() {
        let filters = ConfigurationFilters {
            active_directories: Some(vec![PathBuf::from("/images/a"), PathBuf::from("/images/b")]),
            scores: None,
            width_range: Some(1..=10),
            height_range: Some(5..=7),
            tags: None,
            colors: None,
        };
        let conf = Configuration {
            root_images_dir: PathBuf::from("/foo"),
            metadata_path: PathBuf::from("/meta.json"),
            filters,
        };
        let toml_str = toml::to_string(&conf).unwrap();
        let de: Configuration = toml::from_str(&toml_str).unwrap();
        assert_eq!(de.root_images_dir, conf.root_images_dir);
        assert_eq!(de.filters.width_range, Some(1..=10));
    }

    #[test]
    fn test_json_serialization() {
        let filters = ConfigurationFilters {
            active_directories: None,
            scores: None,
            width_range: Some(1..=10),
            height_range: Some(5..=7),
            tags: None,
            colors: None,
        };
        let conf = Configuration {
            root_images_dir: PathBuf::from("/foo"),
            metadata_path: PathBuf::from("/meta.json"),
            filters,
        };
        let json_str = serde_json::to_string(&conf).unwrap();
        let de: Configuration = serde_json::from_str(&json_str).unwrap();
        assert_eq!(de.filters.width_range, Some(1..=10));
    }
}
