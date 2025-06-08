#![allow(clippy::missing_errors_doc)]

//! Functions for handling temporary file upload through Litterbox's API
//!
//! Calls API described at <https://litterbox.catbox.moe/tools.php>.
//!
//! See <https://litterbox.catbox.moe/faq.php> for allowed filetypes and content.

use anyhow::Result;
use reqwest::{ Client, multipart::{ Form, Part } };

use crate::{ LITTER_API_URL, UASTRING, helper::{ file_name, file_stream } };

/// Upload a temporary file to litterbox.
/// Max size 1GB.
///
/// See <https://litterbox.catbox.moe/faq.php> for allowed formats and content.
///
/// # Arguments
///
/// * `file_path` - Path to the file to be uploaded
/// * `time` - Time until expiration. Valid values are 1, 12, 24 and 72.
pub async fn upload<S: Into<String>>(file_path: S, time: u8) -> Result<String> {
    let file_path = file_path.into();
    let file = file_stream(&file_path).await?;
    let file_name = file_name(&file_path);

    let form = Form::new()
        .text("reqtype", "fileupload")
        .text("time", format!("{time}h"))
        .part("fileToUpload", Part::stream(file).file_name(file_name));

    Ok(
        Client::builder()
            .user_agent(UASTRING)
            .build()
            .unwrap_or_else(|_| Client::new())
            .post(LITTER_API_URL)
            .multipart(form)
            .send().await?
            .text().await?
    )
}
