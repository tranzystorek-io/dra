use std::path::PathBuf;

use clap::{Parser, ValueHint};

use crate::cli::parse_repository::try_parse_repository;
use crate::github::Repository;

/// A command line tool to download release assets from GitHub
///
/// EXAMPLES:
/// Select and download an asset:
/// $ dra download devmatteini/dra-tests
///
/// Automatically select and download an asset:
/// $ dra untag devmatteini/dra-tests
/// helloworld_{tag}.tar.gz
/// $ dra download --select "helloworld_{tag}.tar.gz" devmatteini/dra-tests
///
/// Download and install an asset:
/// $ dra download --install devmatteini/dra-tests
///
/// More examples can be found at https://github.com/devmatteini/dra#usage
#[derive(Debug, Parser)]
#[command(name = "dra", version, verbatim_doc_comment)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Select and download an asset
    Download {
        /// Github repository using format {owner}/{repo}
        #[arg(value_parser = try_parse_repository)]
        repo: Repository,

        /// Untagged asset name to automatically select which asset to download
        ///
        /// Usually this value is generated by the command `untag`
        #[arg(short = 's', long = "select")]
        select: Option<String>,

        /// Set the tag name for fetching a specific release
        ///
        /// Latest release is used if not specified
        #[arg(short = 't', long = "tag")]
        tag: Option<String>,

        /// Save asset to custom path
        ///
        /// Default path is current working directory and the name of the asset.
        ///
        /// If used with `--install` it must be a directory path
        #[arg(short = 'o', long = "output", value_hint = ValueHint::AnyPath)]
        output: Option<PathBuf>,

        /// Install downloaded asset
        ///
        /// If the downloaded asset is a supported file, it will try to install it.
        /// By default is not used.
        #[arg(short = 'i', long = "install")]
        install: bool,
    },

    /// Select an asset and generate an untagged version of it
    Untag {
        /// Github repository using format {owner}/{repo}
        #[arg(value_parser = try_parse_repository)]
        repo: Repository,
    },

    /// Generate shell completion
    Completion {
        /// Shell to generate completion for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}
