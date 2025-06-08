#![allow(clippy::missing_errors_doc)]

//! Functions for handling file upload and deletion through Catbox's API
//!
//! Calls API described at <https://catbox.moe/tools.php>.
//!
//! See <https://catbox.moe/faq.php> for allowed filetypes and content.

use anyhow::Result;
use reqwest::{ Client, multipart::{ Form, Part } };

#[allow(clippy::wildcard_imports)]
use crate::{ CATBOX_API_URL, UASTRING, helper::* };

/// Upload a file to catbox.
///
/// Returns an URL to the file
///
/// See <https://catbox.moe/faq.php> for allowed formats and content.
///
/// # Arguments
///
/// * `file_path` - Path to the file to be uploaded
/// * `user_hash` - User account hash, required for deletions. Otherwise optional.
pub async fn from_file<S: Into<String>>(file_path: S, user_hash: S) -> Result<String> {
    let file_path = file_path.into();
    let file = file_stream(&file_path).await?;
    let file_name = file_name(&file_path);

    let form = Form::new()
        .text("reqtype", "fileupload")
        .text("userhash", user_hash.into())
        .part("fileToUpload", Part::stream(file).file_name(file_name));

    Ok(
        Client::builder()
            .user_agent(UASTRING)
            .build()
            .unwrap_or_else(|_| Client::new())
            .post(CATBOX_API_URL)
            .multipart(form)
            .send().await?
            .text().await?
    )
}

/// Upload contents from a URL to catbox
///
/// Returns n URL to the file
///
/// See <https://catbox.moe/faq.php> for allowed formats and content.
///
/// # Arguments
///
/// * `url` - URL to file
/// * `user_hash` - User account hash, required for deletions. Otherwise optional.
pub async fn from_url<S: Into<String>>(url: S, user_hash: S) -> Result<String> {
    let form = [
        ("reqtype", "urlupload"),
        ("userhash", &user_hash.into()),
        ("url", &url.into()),
    ];

    Ok(
        Client::builder()
            .user_agent(UASTRING)
            .build()
            .unwrap_or_else(|_| Client::new())
            .post(CATBOX_API_URL)
            .form(&form)
            .send().await?
            .text().await?
    )
}

/// Delete files
///
/// Returns "Files successfully deleted." on success
///
/// # Arguments
///
/// * `files` - Names of the files to be deleted
/// * `user_hash` - User account hash, required for deletions. Otherwise optional.
pub async fn delete<S: Into<String>>(files: Vec<S>, user_hash: S) -> Result<String> {
    let files: Vec<_> = files.into_iter().map(Into::into).collect();

    let form = [
        ("reqtype", "deletefiles"),
        ("userhash", &user_hash.into()),
        ("files", &files.join(" ")),
    ];

    Ok(
        Client::builder()
            .user_agent(UASTRING)
            .build()
            .unwrap_or_else(|_| Client::new())
            .post(CATBOX_API_URL)
            .form(&form)
            .send().await?
            .text().await?
    )
}
