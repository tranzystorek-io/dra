use crate::cli::parse_repository::try_parse_repository;
use crate::github::Repository;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(
    name = "dra",
    version,
    about = "A command line tool to download release assets from GitHub"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Command,

    /// Github repository using format {owner}/{repo}
    #[clap(parse(try_from_str = try_parse_repository))]
    pub repo: Repository,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Select and download an asset
    Download {
        /// Untagged asset name to automatically select which asset to download
        ///
        /// Usually this value is generated by the command `untag`
        #[clap(short = 's', long = "select")]
        select: Option<String>,

        /// Set the tag name for fetching a specific release
        ///
        /// Latest release is used if not specified
        #[clap(short = 't', long = "tag")]
        tag: Option<String>,

        /// Save asset to custom path
        ///
        /// Default path is current working directory and the name of the asset.
        ///
        /// If used with `--install` it must be a directory path
        #[clap(short = 'o', long = "output", parse(from_os_str))]
        output: Option<PathBuf>,

        /// Install downloaded asset
        ///
        /// If the downloaded asset is a supported file, it will try to install it.
        /// By default is not used.
        #[clap(short = 'i', long = "install")]
        install: bool,
    },

    /// Select an asset and generate an untagged version of it
    Untag,
}
