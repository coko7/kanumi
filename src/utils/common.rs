use anyhow::{bail, Context, Result};
use directories::ProjectDirs;
use log::{debug, info, warn};
use serde_json::to_string;
use std::{
    env,
    fs::{self, File},
    io::Read,
    ops::RangeInclusive,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};
use yansi::{Color, Paint};

use crate::models::{Configuration, ImageMeta, ScoreFilter};

pub const APP_NAME: &str = "kanumi";
pub const CONFIG_VAR: &str = "KANUMI_CONFIG";

pub fn get_config_dir() -> Result<PathBuf> {
    if let Ok(config_var) = env::var(CONFIG_VAR) {
        let val = PathBuf::from(config_var);
        info!(
            "get config from env: {} = {}",
            CONFIG_VAR,
            val.to_string_lossy()
        );

        return Ok(val);
    }

    if let Some(proj_dirs) = ProjectDirs::from("", "", APP_NAME) {
        let config_dir = proj_dirs.config_dir();
        info!("get config dir from proj dirs: {}", config_dir.display());
        return Ok(config_dir.to_path_buf());
    }

    bail!("could not get config directory")
}

pub fn get_config_file() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.toml"))
}

pub fn create_config_file() -> Result<()> {
    let file_path = get_config_file()?;
    info!("create config file: `{}`", file_path.to_string_lossy());

    if let Some(config_dir) = file_path.parent() {
        fs::create_dir_all(config_dir)?;
    }

    let default_config = Configuration::create_default();
    let toml = default_config.to_toml_str()?;

    fs::write(&file_path, toml)?;
    Ok(())
}

pub fn load_config(path: PathBuf) -> Result<Configuration> {
    let content = fs::read_to_string(path)?;

    info!("parsing config toml");
    let config: Configuration = toml::from_str(&content)?;
    Ok(config)
}

pub fn parse_score_filters(input: &str) -> Result<ScoreFilter> {
    let mut allow_unscored = false;
    let mut input = input.to_string();

    if input.ends_with('@') {
        input = input
            .strip_suffix('@')
            .context("failed to strip ! suffix on score filter")?
            .to_owned();

        allow_unscored = true;
    }

    let (key, range) = input
        .split_once('=')
        .context("failed to split input on '='")?;

    let score_filter = ScoreFilter {
        name: key.to_string(),
        range: parse_range(range)?,
        allow_unscored,
    };

    Ok(score_filter)
}

pub fn parse_range(input: &str) -> Result<RangeInclusive<usize>> {
    if let Ok(num) = input.parse::<usize>() {
        return Ok(num..=num);
    }

    if !input.contains("..") {
        bail!("expected number N or range (N..O) but got: `{}`", input);
    }

    let parts: Vec<&str> = input.split("..").collect();
    if parts.len() != 2 {
        bail!("invalid range format, expected X..Y but got: `{}`", input);
    }

    let mut formatted_parts = Vec::new();
    for part in parts {
        if part.is_empty() {
            formatted_parts.push(None);
        } else {
            match part.parse::<usize>() {
                Ok(num) => formatted_parts.push(Some(num)),
                Err(e) => bail!("failed to parse number: `{}`", e),
            }
        }
    }

    match formatted_parts.as_slice() {
        [None, Some(end)] => Ok(0..=*end),
        [Some(start), None] => Ok(*start..=usize::MAX),
        [Some(start), Some(end)] => {
            if start > end {
                bail!("start should be <= end: {} > {}", start, end);
            }
            Ok(*start..=*end)
        }
        _ => bail!("range should have at least one boundary"),
    }
}

pub fn get_image_dims(image: &PathBuf) -> Result<(u32, u32)> {
    Ok(image::image_dimensions(image)?)
}

pub fn image_matches_dims(
    image: &PathBuf,
    width_range: &Option<RangeInclusive<usize>>,
    height_range: &Option<RangeInclusive<usize>>,
) -> bool {
    debug!("checking dimensions for: {}", image.display());
    let dimensions = image::image_dimensions(image);
    if let Err(error) = dimensions {
        warn!(
            "failed to check dimensions for: {}, error: {}",
            image.display(),
            error
        );
        return false;
    }

    let dimensions = dimensions.unwrap();
    let (width, height) = (dimensions.0 as usize, dimensions.1 as usize);

    if let Some(width_range) = width_range {
        if !width_range.contains(&width) {
            return false;
        }
    }

    if let Some(height_range) = height_range {
        if !height_range.contains(&height) {
            return false;
        }
    }

    true
}

pub fn image_score_matches(meta: &ImageMeta, score_filter: &ScoreFilter) -> bool {
    let img_score = meta
        .scores
        .iter()
        .find(|score| score.name == score_filter.name);

    if let Some(img_score) = img_score {
        return usize::from(img_score.value) >= *score_filter.range.start()
            && usize::from(img_score.value) <= *score_filter.range.end();
    }

    score_filter.allow_unscored
}

pub fn load_image_metas(meta_file_path: &Path) -> Result<Vec<ImageMeta>> {
    let data = fs::read_to_string(meta_file_path)?;
    let metas: Vec<ImageMeta> = serde_json::from_str(&data)?;
    Ok(metas)
}

pub fn get_all_images(base_directory: &Path) -> Result<Vec<PathBuf>> {
    Ok(WalkDir::new(base_directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_image_file)
        .map(|entry| entry.path().to_owned())
        .collect())
}

fn is_image_file(entry: &DirEntry) -> bool {
    if let Some(file_name) = entry.file_name().to_str() {
        return file_name.to_lowercase().ends_with(".gif")
            || file_name.to_lowercase().ends_with(".jpeg")
            || file_name.to_lowercase().ends_with(".jpg")
            || file_name.to_lowercase().ends_with(".png")
            || file_name.to_lowercase().ends_with(".webp");
    }

    false
}

pub fn compute_blake3_hash(file: &Path) -> Result<String> {
    let mut file = File::open(file)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(hash.to_string())
}

pub fn create_banner(text: &str, border_char: char) -> String {
    let center_part = format!("{border_char} {text} {border_char}\n");

    let inside_len = center_part.len() - 3;
    let outline = format!(
        "{border_char}{}{border_char}\n",
        border_char.to_string().repeat(inside_len)
    );
    let empty = format!("{border_char}{}{border_char}\n", " ".repeat(inside_len));

    format!("{outline}{empty}{center_part}{empty}{outline}")
}

pub fn colorize_rainbow(text: &str, chunk_len: usize) -> String {
    const COLORS: [Color; 7] = [
        Color::Red,
        Color::Rgb(255, 165, 0),
        Color::Yellow,
        Color::Green,
        Color::Cyan,
        Color::Blue,
        Color::Magenta,
    ];

    text.chars()
        .enumerate()
        .map(|(i, c)| {
            let chunked_idx = i / chunk_len;
            let color = &COLORS[chunked_idx % COLORS.len()];
            Paint::new(c.to_string()).fg(*color).to_string()
        })
        .collect()
}

pub fn get_image_by_path_or_id<'a>(
    identifier: &str,
    metadatas: &'a [ImageMeta],
) -> Result<Option<&'a ImageMeta>> {
    let path = Path::new(identifier);
    if let Some(meta) = metadatas.iter().find(|m| m.path == path) {
        return Ok(Some(meta));
    }

    Ok(metadatas.iter().find(|m| m.id == identifier))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range_valid_integers() {
        assert_eq!(parse_range("5").unwrap(), 5..=5);
        assert_eq!(parse_range("3..7").unwrap(), 3..=7);
    }

    #[test]
    fn test_parse_range_open_ranges() {
        assert_eq!(parse_range("..7").unwrap(), 0..=7);
        assert_eq!(parse_range("3..",).unwrap(), 3..=usize::MAX);
    }

    #[test]
    fn test_parse_range_invalid() {
        assert!(parse_range("7..3").is_err());
        assert!(parse_range("notanumber").is_err());
        assert!(parse_range("").is_err());
        assert!(parse_range("3...5").is_err());
    }

    #[test]
    fn test_parse_score_filters() {
        let filter = parse_score_filters("foo=1..10").unwrap();
        assert_eq!(filter.name, "foo");
        assert_eq!(filter.range, 1..=10);
        assert!(!filter.allow_unscored);
    }
    #[test]
    fn test_parse_score_filter_with_at() {
        let filter = parse_score_filters("foo=2@").unwrap();
        assert_eq!(filter.name, "foo");
        assert_eq!(filter.range, 2..=2);
        assert!(filter.allow_unscored);
    }

    #[test]
    fn test_parse_range_more_edge_cases() {
        // Large numbers
        assert_eq!(parse_range("0..100000").unwrap(), 0..=100000);
        // Zero-width
        assert_eq!(parse_range("0..0").unwrap(), 0..=0);
        // Spaces and invalid
        assert!(parse_range("   ").is_err());
        // Open-ended invalid
        assert!(parse_range("..").is_err());
    }

    #[test]
    fn test_parse_score_filters_errors() {
        // Missing '='
        assert!(parse_score_filters("foo10").is_err());
        // Bad range
        assert!(parse_score_filters("foo=a..b").is_err());
        // Empty
        assert!(parse_score_filters("").is_err());
        // Double '='
        assert!(parse_score_filters("foo=10=12").is_err());
    }

    #[test]
    fn test_image_score_matches() {
        let filter = ScoreFilter {
            name: "qual".to_string(),
            range: 30..=40,
            allow_unscored: false,
        };
        let meta_scored = ImageMeta {
            id: "id".to_string(),
            path: std::path::PathBuf::from("image.png"),
            scores: vec![crate::models::image_meta::ImageScore {
                name: "qual".to_string(),
                value: 35,
            }],
            ..Default::default()
        };
        let meta_no_score = ImageMeta {
            id: "id".to_string(),
            path: std::path::PathBuf::from("image.png"),
            scores: vec![],
            ..Default::default()
        };
        assert!(super::image_score_matches(&meta_scored, &filter));
        assert!(!super::image_score_matches(&meta_no_score, &filter));
        let filter_allow = ScoreFilter {
            allow_unscored: true,
            ..filter
        };
        assert!(super::image_score_matches(&meta_no_score, &filter_allow));
    }

    // (You might want quickcheck/arbitrary for prop-tests, but here's one more demonstrative case)
    #[test]
    fn test_parse_range_fail_on_float() {
        assert!(parse_range("1.5..7.2").is_err());
        assert!(parse_range("4.2").is_err());
    }
}
