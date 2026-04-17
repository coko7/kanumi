use anyhow::Result;
use log::info;
use std::path::Path;

use super::ConfigurationCommands;
use crate::{models::Configuration, utils};

pub fn handle_config_command(
    command: ConfigurationCommands,
    configuration: &Configuration,
) -> Result<()> {
    match command {
        ConfigurationCommands::Show(display_format) => {
            let config_path = utils::common::get_config_file()?;

            if display_format.json {
                show_config_as_json(configuration)
            } else if display_format.toml {
                show_config_as_toml(configuration, &config_path)
            } else {
                show_config_with_default_formatter(configuration, &config_path)
            }
        }
        ConfigurationCommands::Generate { dry_run: _ } => {
            info!("generating default config...");
            let default_config = Configuration::create_default();
            let toml = default_config.to_toml_str()?;
            print!("{}", toml);
            Ok(())
        }
    }
}

fn show_config_as_json(configuration: &Configuration) -> Result<()> {
    let json_config = serde_json::to_string(configuration)?;
    println!("{json_config}");
    Ok(())
}

fn show_config_as_toml(configuration: &Configuration, config_path: &Path) -> Result<()> {
    let banner = utils::common::create_banner(&config_path.display().to_string(), '*');
    let rainbow_banner = utils::common::colorize_rainbow(&banner, 2);
    println!("{rainbow_banner}");

    let toml_config = configuration.to_toml_str()?;
    println!("{toml_config}");
    Ok(())
}

fn show_config_with_default_formatter(
    configuration: &Configuration,
    config_path: &Path,
) -> Result<()> {
    show_config_as_toml(configuration, config_path)
}
