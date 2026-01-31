use clap::{command, ArgGroup, Args, Parser, Subcommand};
use std::{ffi::OsString, ops::RangeInclusive, path::PathBuf};

use crate::{
    models::ScoreFilter,
    utils::common::{parse_range, parse_score_filters},
};

#[derive(Debug, Parser)]
#[command(name = "kanumi")]
#[command(about = "Manage collection of images from your terminal", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// View and manage configuration
    #[command(name = "config", alias = "cfg")]
    Configuration {
        #[command(subcommand)]
        command: ConfigurationCommands,
    },
    /// View and manage metadata
    #[command(name = "metadata", alias = "meta")]
    Metadata {
        #[command(subcommand)]
        command: MetadataCommands,
    },
    /// List all directories containing images
    #[command(name = "dirs")]
    Dirs {
        /// Output in JSON
        #[arg(short = 'j', long = "json")]
        use_json_format: bool,
    },
    /// List images that match given selectors
    #[command(name = "list", alias = "ls")]
    List {
        /// Filter based on parent directories
        #[arg(short = 'd', long = "directories")]
        active_directories: Option<Vec<PathBuf>>,

        /// Filter based on score range
        #[arg(short = 's', long = "scores", value_parser = parse_score_filters)]
        scores: Option<Vec<ScoreFilter>>,

        /// Filter based on width range
        #[arg(short = 'W', long = "width", value_parser = parse_range)]
        width_range: Option<RangeInclusive<usize>>,

        /// Filter based on height range
        #[arg(short = 'H', long = "height", value_parser = parse_range)]
        height_range: Option<RangeInclusive<usize>>,

        #[arg(short = 't', long = "tags")]
        tags: Option<Vec<String>>,

        /// Ignore selectors preset from config
        #[arg(short = 'i', long = "ignore")]
        ignore_config: bool,

        /// Output in JSON
        #[arg(short = 'j', long = "json")]
        use_json_format: bool,
    },
    /// Scan the entire images directory to find missing data
    Scan {
        /// Output in JSON
        #[arg(short = 'j', long = "json")]
        use_json_format: bool,
    },
}

#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("format")
        .args(["json", "toml"])
        .multiple(false)
))]
pub struct ConfigShowFormatArgs {
    /// Output in JSON
    #[arg(short = 'j', long)]
    pub json: bool,

    /// Output in TOML (default)
    #[arg(short = 't', long)]
    pub toml: bool,
}

#[derive(Debug, Subcommand)]
pub enum ConfigurationCommands {
    /// Print configuration and exit
    Show(ConfigShowFormatArgs),

    /// Generate a default configuration file
    #[command(visible_alias = "gen")]
    Generate {
        /// Only print generated configuration. Does not write to file system
        #[arg(short, long)]
        dry_run: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum MetadataCommands {
    /// Print all metadatas and exit
    Show,
    /// Get the metadata associated to a given image file
    Get {
        /// Metadata ID or path of the image file
        identifier: OsString,
    },
    /// Search for metadata using a search string
    Search {
        /// The search query
        query: OsString,

        /// Output in JSON
        #[arg(short = 'j', long = "json")]
        use_json_format: bool,
    },
    /// Update the metadata for an image
    Edit {
        /// Metadata ID or path of the image file
        identifier: OsString,

        /// Updated metadata string in JSON format
        payload: OsString,
    },
    /// Generate default metadata for a given image
    #[command(visible_alias = "gen")]
    Generate {
        /// Path of the image file
        image: PathBuf,

        /// Only print generated configuration. Does not write to file system
        #[arg(short, long)]
        dry_run: bool,
    },
    // /// Generate metadata file based on configured images directory
    // #[command(visible_alias = "gen-meta")]
    // GenerateMetadata { image: PathBuf },
}
