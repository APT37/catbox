#![allow(clippy::missing_errors_doc)]

use anyhow::Result;
use clap::Parser;
use futures::stream::{ FuturesUnordered, StreamExt };
use regex::Regex;
use std::{ path::Path, sync::LazyLock };
use url::Url;

use args::{
    Album,
    AlbumAdd,
    AlbumCommand,
    AlbumCreate,
    AlbumDelete,
    AlbumEdit,
    AlbumRemove,
    CatboxArgs,
    CatboxCommand,
    Delete,
    Litter,
    Upload,
};

#[cfg(not(test))]
use catbox::{ album, file, litter };
#[cfg(test)]
mod test;
#[cfg(test)]
use test::catbox::{ album, file, litter };

mod args;

#[tokio::main]
async fn main() -> Result<()> {
    match CatboxArgs::try_parse() {
        Ok(args) =>
            match args.command {
                CatboxCommand::Upload(sub_args) => upload(sub_args).await,
                CatboxCommand::Delete(sub_args) => delete_file(sub_args).await,
                CatboxCommand::Album(sub_args) => parse_album(sub_args).await,
                CatboxCommand::Litter(sub_args) => litter(sub_args).await,
            }
        Err(args) => {
            println!("{args}");

            Ok(())
        }
    }
}

static RE_CATBOX: LazyLock<Regex> = LazyLock::new(||
    Regex::new(r"^(https?://)?files.catbox.moe/.+").unwrap()
);

fn catbox_url_to_image_name(url: &str) -> String {
    (if RE_CATBOX.is_match(url) { url.split('/').next_back().unwrap() } else { url }).to_string()
}

fn album_url_to_short(url: &str) -> String {
    (if RE_CATBOX.is_match(url) { url.split('/').next_back().unwrap() } else { url }).to_string()
}

async fn parse_album(album_args: Album) -> Result<()> {
    match album_args.album_command {
        AlbumCommand::Create(sub_args) => create_album(sub_args).await,
        AlbumCommand::Delete(sub_args) => delete_album(sub_args).await,
        AlbumCommand::Edit(sub_args) => edit_album(sub_args).await,
        AlbumCommand::Add(sub_args) => add_to_album(sub_args).await,
        AlbumCommand::Remove(sub_args) => remove_from_album(sub_args).await,
    }
}

async fn upload(upload_args: Upload) -> Result<()> {
    let (files, rest): (Vec<_>, _) = upload_args.files
        .into_iter()
        .partition(|uri| Path::new(&uri).exists());
    let (urls, rest): (Vec<_>, _) = rest.iter().partition(|uri| Url::parse(uri).is_ok());
    let user = upload_args.user_hash;
    let print_result = |res| async move { println!("{res}") };

    tokio::join!(
        rest
            .into_iter()
            .map(|uri| invalid_uri(uri.to_string()))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_result),
        urls
            .into_iter()
            .map(|url| upload_url(url.to_string(), &user))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_result),
        files
            .into_iter()
            .map(|file| upload_file(file, &user))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_result)
    );

    Ok(())
}

async fn invalid_uri(uri: String) -> String {
    format!("Ignoring {uri}: invalid path or URL")
}

async fn upload_file(file: String, user_hash: &String) -> String {
    file::from_file(file.clone(), user_hash.to_string()).await.unwrap_or_else(|_|
        format!("Uploading {file} failed.")
    )
}

async fn upload_url(url: String, user_hash: &String) -> String {
    file::from_url(&url, user_hash).await.unwrap_or_else(|_| format!("Uploading {url} failed."))
}

async fn upload_to_litter(file_path: String, time: u8) -> String {
    litter
        ::upload(&file_path, time).await
        .unwrap_or_else(|_| format!("Uploading {file_path} failed."))
}

async fn delete_file(delete_args: Delete) -> Result<()> {
    let res = file::delete(
        delete_args.files
            .into_iter()
            .map(|file| catbox_url_to_image_name(&file))
            .collect(),
        delete_args.user_hash
    ).await?;

    println!("{res}");

    Ok(())
}

async fn litter(litter_args: Litter) -> Result<()> {
    let (files, rest): (Vec<_>, _) = litter_args.files
        .into_iter()
        .partition(|path| Path::new(&path).exists());

    let print_res = |res| async move { println!("{res}") };

    tokio::join!(
        rest
            .into_iter()
            .map(invalid_uri)
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_res),
        files
            .into_iter()
            .map(|file| upload_to_litter(file, litter_args.time.unwrap_or(1)))
            .collect::<FuturesUnordered<_>>()
            .for_each_concurrent(10, print_res)
    );

    Ok(())
}

async fn create_album(album_create_args: AlbumCreate) -> Result<()> {
    let res = album::create(
        album_create_args.title,
        album_create_args.description.unwrap_or_default(),
        album_create_args.user_hash,
        album_create_args.files
            .into_iter()
            .map(|file| catbox_url_to_image_name(&file))
            .collect()
    ).await?;

    println!("{res}");

    Ok(())
}

async fn delete_album(album_delete_args: AlbumDelete) -> Result<()> {
    let res = album::delete(
        album_url_to_short(&album_delete_args.short),
        album_delete_args.user_hash
    ).await?;

    println!("{res}");

    Ok(())
}

async fn edit_album(album_edit_args: AlbumEdit) -> Result<()> {
    let res = album::edit(
        album_url_to_short(&album_edit_args.short),
        album_edit_args.title,
        album_edit_args.description.unwrap_or_default(),
        album_edit_args.user_hash,
        album_edit_args.files
            .into_iter()
            .map(|file| catbox_url_to_image_name(&file))
            .collect()
    ).await?;

    println!("{res}");

    Ok(())
}

async fn add_to_album(album_add_args: AlbumAdd) -> Result<()> {
    let res = album::add_files(
        album_url_to_short(&album_add_args.short),
        album_add_args.user_hash,
        album_add_args.files
            .into_iter()
            .map(|file| catbox_url_to_image_name(&file))
            .collect()
    ).await?;

    println!("{res}");

    Ok(())
}

async fn remove_from_album(album_remove_args: AlbumRemove) -> Result<()> {
    let res = album::remove_files(
        album_url_to_short(&album_remove_args.short),
        album_remove_args.user_hash,
        album_remove_args.files
            .into_iter()
            .map(|file| catbox_url_to_image_name(&file))
            .collect()
    ).await?;

    println!("{res}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::Builder;

    static FILE_URL: &str =
        "https://file-examples.com/wp-content/storage/2017/10/file_example_JPG_100kB.jpg";

    #[tokio::test]
    async fn upload_file() -> Result<()> {
        let mut file = Builder::new().suffix(".txt").tempfile().unwrap();
        write!(file, "content").unwrap();

        let args = CatboxArgs::parse_from(
            vec!["catbox", "upload", "--user", "123456", file.path().to_str().unwrap()]
        );

        if let CatboxCommand::Upload(upload_args) = args.command {
            upload(upload_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn upload_url() -> Result<()> {
        let args = CatboxArgs::parse_from(vec!["catbox", "upload", "--user", "123456", FILE_URL]);

        if let CatboxCommand::Upload(upload_args) = args.command {
            upload(upload_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn nonexistant() -> Result<()> {
        let args = CatboxArgs::parse_from(
            vec!["catbox", "upload", "--user", "123456", "This is not a file or url"]
        );

        if let CatboxCommand::Upload(upload_args) = args.command {
            upload(upload_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn upload_multi() -> Result<()> {
        let mut file = Builder::new().suffix(".txt").tempfile().unwrap();
        write!(file, "content").unwrap();

        let args = CatboxArgs::parse_from(
            vec![
                "catbox",
                "upload",
                "--user",
                "123456",
                file.path().to_str().unwrap(),
                FILE_URL,
                "Something",
                "Something else"
            ]
        );

        if let CatboxCommand::Upload(upload_args) = args.command {
            upload(upload_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn delete_files() -> Result<()> {
        let args = CatboxArgs::parse_from(
            vec!["catbox", "delete", "--user", "123456", "file.png", "another.jpg"]
        );

        if let CatboxCommand::Delete(delete_args) = args.command {
            delete_file(delete_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn album_create() -> Result<()> {
        let args = CatboxArgs::parse_from(
            vec![
                "catbox",
                "album",
                "create",
                "--desc",
                "A description",
                "--title",
                "My album",
                "--user",
                "123456",
                "file.png",
                "another.jpg"
            ]
        );

        if let CatboxCommand::Album(album_args) = args.command {
            parse_album(album_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn album_add() -> Result<()> {
        let args = CatboxArgs::parse_from(
            vec!["catbox", "album", "add", "--user", "123456", "--short", "123asd", "file.png"]
        );

        if let CatboxCommand::Album(album_args) = args.command {
            parse_album(album_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn album_remove() -> Result<()> {
        let args = CatboxArgs::parse_from(
            vec!["catbox", "album", "remove", "--user", "123456", "--short", "123asd", "file.png"]
        );

        if let CatboxCommand::Album(album_args) = args.command {
            parse_album(album_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn album_delete() -> Result<()> {
        let args = CatboxArgs::parse_from(
            vec!["catbox", "album", "delete", "--user", "123345", "asd123"]
        );

        if let CatboxCommand::Album(album_args) = args.command {
            parse_album(album_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn album_edit() -> Result<()> {
        let args = CatboxArgs::parse_from(
            vec![
                "catbox",
                "album",
                "edit",
                "--desc",
                "A description",
                "--title",
                "My Album",
                "--user",
                "123345",
                "--short",
                "asd123",
                "file.png",
                "another.jpg"
            ]
        );

        if let CatboxCommand::Album(album_args) = args.command {
            parse_album(album_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    async fn upload_litter() -> Result<()> {
        let mut file = Builder::new().suffix(".txt").tempfile().unwrap();
        write!(file, "content").unwrap();

        let args = CatboxArgs::parse_from(
            vec!["catbox", "litter", "--time", "1", file.path().to_str().unwrap()]
        );

        if let CatboxCommand::Litter(litter_args) = args.command {
            litter(litter_args).await?;
        } else {
            panic!("Invalid subcommand");
        }

        Ok(())
    }

    #[tokio::test]
    #[should_panic = "Invalid subcommand"]
    async fn invalid_command() {
        if let Ok(args) = CatboxArgs::try_parse() {
            if let CatboxCommand::Album(album_args) = args.command {
                let _ = parse_album(album_args).await;
            }
        } else {
            panic!("Invalid subcommand")
        }
    }
}
