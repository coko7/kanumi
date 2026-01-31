use anyhow::{ensure, Result};
use clap::Parser;
use cli::{Cli, Commands};
use log::{error, info, warn};
use std::process::ExitCode;

mod cli;
mod models;
mod utils;

fn main() -> ExitCode {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("process cli args");
    match process_args(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            error!("{}", e);
            ExitCode::FAILURE
        }
    }
}

fn process_args(args: Cli) -> Result<()> {
    info!("getting config file");
    let config_file = utils::common::get_config_file()?;
    if !config_file.exists() {
        utils::common::create_config_file()?;
        info!("config file created");
    }

    info!("loading config");
    let config = utils::common::load_config(config_file)?;

    ensure!(
        config.root_images_dir.exists(),
        "could not find root images directory: {}",
        config.root_images_dir.display()
    );
    ensure!(
        config.metadata_path.exists(),
        "could not find metadata file: {}",
        config.metadata_path.display()
    );

    info!("metadata_path: {:?}", config.metadata_path);
    match args.command {
        Commands::List {
            active_directories,
            scores,
            width_range,
            height_range,
            tags,
            ignore_config,
            use_json_format,
        } => {
            let mut active_directories = active_directories;
            let mut score_filters = scores;
            let mut width_range = width_range;
            let mut height_range = height_range;

            if !ignore_config {
                active_directories = active_directories.or(config.filters.active_directories);
                score_filters = score_filters.or(config.filters.scores);
                width_range = width_range.or(config.filters.width_range);
                height_range = height_range.or(config.filters.height_range);
            } else {
                info!("ignore_config flag has been added");
            }

            warn!("right now, metadata file is required to list images");
            cli::list_images_using_metadata(
                &config.root_images_dir,
                &config.metadata_path,
                active_directories,
                score_filters,
                width_range,
                height_range,
                tags,
                use_json_format,
            )
        }
        cli::Commands::Scan { use_json_format } => cli::scan_images(
            &config.root_images_dir,
            &config.metadata_path,
            use_json_format,
        ),
        cli::Commands::Configuration { command } => cli::handle_config_command(command, &config),
        cli::Commands::Metadata { command } => cli::handle_metadata_command(command, &config),
        cli::Commands::Dirs { use_json_format } => {
            cli::handle_dirs_command(use_json_format, &config)
        }
    }
}
