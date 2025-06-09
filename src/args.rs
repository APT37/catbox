#![allow(clippy::missing_errors_doc)]

use anyhow::{ Result, bail };
use clap::{ Args, Parser, Subcommand };

#[derive(Debug, PartialEq, Subcommand)]
pub enum CatboxCommand {
    Upload(Upload),
    Delete(Delete),
    Album(Album),
    Litter(Litter),
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum AlbumCommand {
    Create(AlbumCreate),
    Add(AlbumAdd),
    Edit(AlbumEdit),
    Remove(AlbumRemove),
    Delete(AlbumDelete),
}

#[derive(Debug, Parser)]
#[command(about = "Unofficial Catbox.moe CLI", version)]
pub struct CatboxArgs {
    #[command(subcommand)]
    pub command: CatboxCommand,

    #[arg(
        global = true,
        short,
        long = "user",
        help = "Catbox user hash",
        env = "CATBOX_USER_HASH"
    )]
    pub user_hash: Option<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Upload to Catbox (max. 200MB)", arg_required_else_help(true))]
pub struct Upload {
    #[arg(from_global)]
    pub user_hash: String,

    #[arg(num_args(1..), help = "File paths or URLs")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Delete files", arg_required_else_help(true))]
pub struct Delete {
    #[arg(from_global)]
    pub user_hash: String,

    #[arg(num_args(1..), help = "File IDs")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Album commands", arg_required_else_help(true))]
pub struct Album {
    #[command(subcommand)]
    pub album_command: AlbumCommand,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Upload a temporary file to Litterbox (max. 1GB)", arg_required_else_help(true))]
pub struct Litter {
    #[arg(short, long, help = "File lifetime in hours", value_parser = valid_hour)]
    pub time: Option<u8>,

    #[arg(num_args(1..), help = "File paths")]
    pub files: Vec<String>,
}

fn valid_hour(hour: &str) -> Result<u8> {
    if let Ok(hour) = hour.parse::<u8>() {
        if [1, 12, 24, 72].contains(&hour) {
            Ok(hour)
        } else {
            bail!("{hour} is not a valid value (Options: 1, 12, 24, 72")
        }
    } else {
        bail!("{hour} is not a valid number");
    }
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Create an album", arg_required_else_help(true))]
pub struct AlbumCreate {
    #[arg(short, long, help = "Album title")]
    pub title: String,

    #[arg(short, long, alias = "desc", help = "Album description")]
    pub description: Option<String>,

    #[arg(from_global)]
    pub user_hash: String,

    #[arg(num_args(1..), help = "File IDs")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Edit an album", arg_required_else_help(true))]
pub struct AlbumEdit {
    #[arg(short, long, help = "Album ID")]
    pub short: String,

    #[arg(short, long, help = "Album title")]
    pub title: String,

    #[arg(short, long, alias = "desc", help = "Album description")]
    pub description: Option<String>,

    #[arg(from_global)]
    pub user_hash: String,

    #[arg(num_args(1..), help = "Album ID")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Add files to an album", arg_required_else_help(true))]
pub struct AlbumAdd {
    #[arg(short, long, help = "Album ID")]
    pub short: String,

    #[arg(from_global)]
    pub user_hash: String,

    #[arg(num_args(1..), help = "Catbox IDs of the files to add to the album")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Remove files from an album", arg_required_else_help(true))]
pub struct AlbumRemove {
    #[arg(short, long, help = "Album ID")]
    pub short: String,

    #[arg(from_global)]
    pub user_hash: String,

    #[arg(num_args(1..), help = "Catbox IDs of the files to remove from the album")]
    pub files: Vec<String>,
}

#[derive(Debug, PartialEq, Args)]
#[command(about = "Delete an album", arg_required_else_help(true))]
pub struct AlbumDelete {
    #[arg(from_global)]
    pub user_hash: String,

    #[arg(help = "Album ID")]
    pub short: String,
}
