use anyhow::Result;
use log::{debug, info};
use std::{ops::RangeInclusive, path::PathBuf};

use crate::{models::ScoreFilter, utils};

pub struct ListArgs {
    pub root_images_dir: PathBuf,
    pub metadata_path: PathBuf,
    pub active_directories: Option<Vec<PathBuf>>,
    pub score_filters: Option<Vec<ScoreFilter>>,
    pub width_range: Option<RangeInclusive<usize>>,
    pub height_range: Option<RangeInclusive<usize>>,
    pub tags: Option<Vec<String>>,
    pub use_json_format: bool,
}

pub fn list_images_using_metadata(args: ListArgs) -> Result<()> {
    let ListArgs {
        root_images_dir,
        metadata_path,
        active_directories,
        score_filters,
        width_range,
        height_range,
        tags,
        use_json_format,
    } = args;

    debug!("loading image metadatas");
    let metas = utils::common::load_image_metas(&metadata_path)?;

    info!("active_directories: {:?}", active_directories);
    info!("score_filters: {:?}", score_filters);
    info!("width_range: {:?}", width_range);
    info!("height_range: {:?}", height_range);

    let mut filtered_metas = vec![];

    if let Some(active_dirs) = active_directories {
        for active_dir in active_dirs.iter() {
            info!("filter using active directory: {:?}", active_dir);
            let matching_metas: Vec<_> = metas
                .iter()
                .filter(|meta| {
                    let base_directory = if active_dir.is_absolute() {
                        active_dir.clone()
                    } else {
                        root_images_dir.join(active_dir)
                    };

                    meta.path
                        .ancestors()
                        .any(|ancestor| ancestor == base_directory)
                })
                .cloned()
                .collect();

            filtered_metas.extend(matching_metas);
        }
    } else {
        info!("no active_dirs filters provided, setting filtered_metas to full list");
        filtered_metas.extend(metas);
    }

    if width_range.is_some() || height_range.is_some() {
        info!("applying dimensions filter...");
        filtered_metas.retain(|meta| {
            utils::common::image_matches_dims(&meta.path, &width_range, &height_range)
        });
    }

    if let Some(score_filters) = score_filters {
        info!("applying image meta score filters...");

        for score_filter in score_filters.iter() {
            filtered_metas.retain(|meta| utils::common::image_score_matches(meta, score_filter));
        }
    }

    if let Some(tags) = tags {
        info!("applying tags filters...");

        for tag in tags.iter() {
            filtered_metas.retain(|meta| meta.tags.contains(tag));
        }
    }

    debug!("about to render output");
    match use_json_format {
        true => {
            info!("outputting as json");
            let metas_json = serde_json::to_string(&filtered_metas)?;
            println!("{}", metas_json);
        }
        false => {
            info!("outputting image paths only");
            for meta in filtered_metas.iter() {
                println!("{}", meta.path.display());
            }
        }
    };

    Ok(())
}

fn filter_images_without_using_metadata(
    base_directory: PathBuf,
    width_range: Option<RangeInclusive<usize>>,
    height_range: Option<RangeInclusive<usize>>,
) -> Result<()> {
    info!("width_range: {:?}", width_range);
    info!("height_range: {:?}", height_range);

    info!("about to run WalkDir on {}", base_directory.display());
    let mut images = utils::common::get_all_images(&base_directory)?;

    if width_range.is_some() || height_range.is_some() {
        info!("applying dimensions filter...");
        images.retain(|img| utils::common::image_matches_dims(img, &width_range, &height_range));
    }

    for image in images.iter() {
        println!("{}", image.display());
    }

    todo!("not fully supported yet!")
}
