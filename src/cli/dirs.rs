use anyhow::Result;
use log::{debug, info};
use std::path::PathBuf;

use crate::{models::Configuration, utils};

pub fn handle_dirs_command(use_json_format: bool, config: &Configuration) -> Result<()> {
    debug!("loading image metadatas");
    let metas = utils::common::load_image_metas(&config.metadata_path)?;

    let mut all_dirs: Vec<PathBuf> = vec![];
    for meta in metas.iter() {
        let path = PathBuf::from(&meta.path);
        if let Some(parent_dir) = path.parent() {
            let parent_dir = parent_dir.to_path_buf();
            if !all_dirs.contains(&parent_dir) {
                all_dirs.push(parent_dir);
            }
        }
    }

    if use_json_format {
        info!("outputting as json");
        let json = serde_json::to_string(&all_dirs)?;
        println!("{}", json);
    } else {
        info!("outputting raw dirs");
        for dir in all_dirs.iter() {
            println!("{}", dir.display());
        }
    }

    Ok(())
}
