use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::utils;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ColorTheme {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Color {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "pink")]
    Pink,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "brown")]
    Brown,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "darkgray")]
    DarkGray,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImageMeta {
    // blake3 hash
    pub id: String,
    pub path: PathBuf,
    pub title: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub scores: Vec<ImageScore>,
    pub tags: Vec<String>,
    pub theme: Option<ColorTheme>,
    pub colors: Vec<Color>,
}

impl ImageMeta {
    pub fn create_from_image(image: &PathBuf) -> Result<ImageMeta> {
        let id = utils::common::compute_blake3_hash(image)?;
        let filename = image
            .file_name()
            .context("image file should have a filename")?
            .to_string_lossy()
            .into_owned();

        let dimensions = utils::common::get_image_dims(image)?;

        let meta = ImageMeta {
            id,
            path: image.to_path_buf(),
            title: filename.to_owned(),
            description: String::from(""),
            width: dimensions.0,
            height: dimensions.1,
            scores: vec![],
            tags: vec![],
            theme: None,
            colors: vec![],
        };

        Ok(meta)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImageScore {
    pub name: String,
    pub value: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization_roundtrip() -> Result<()> {
        let meta = ImageMeta {
            id: "123".to_string(),
            path: PathBuf::from("/path/to/image.jpg"),
            title: "foo".to_string(),
            description: "bar".to_string(),
            width: 100,
            height: 200,
            scores: vec![ImageScore {
                name: "clarity".to_string(),
                value: 42,
            }],
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            theme: Some(ColorTheme::Light),
            colors: vec![Color::Red, Color::Blue],
        };

        let ser = serde_json::to_string(&meta)?;
        let deser: ImageMeta = serde_json::from_str(&ser)?;

        assert_eq!(deser.id, meta.id);
        assert_eq!(deser.width, meta.width);
        assert_eq!(deser.theme, meta.theme);
        assert_eq!(deser.colors, meta.colors);
        assert_eq!(deser.scores[0].name, "clarity");

        Ok(())
    }
}
