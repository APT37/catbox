#[allow(clippy::unused_async)]
pub mod catbox {
    pub mod album {
        use anyhow::Result;

        pub async fn create<S: Into<String>>(
            _title: S,
            _desc: S,
            _user_hash: S,
            _files: Vec<S>
        ) -> Result<String> {
            Ok("https://catbox.moe/c/123435".to_string())
        }

        pub async fn delete<S: Into<String>>(_short: S, _user_hash: S) -> Result<String> {
            Ok(String::new())
        }

        pub async fn edit<S: Into<String>>(
            short: S,
            _title: S,
            _desc: S,
            _user_hash: S,
            _files: Vec<S>
        ) -> Result<String> {
            let short = short.into();

            Ok(
                if short.is_empty() {
                    "No album found for specified user.".to_string()
                } else {
                    format!("https://catbox.moe/c/{short}")
                }
            )
        }

        pub async fn add_files<S: Into<String>>(
            short: S,
            _user_hash: S,
            _files: Vec<S>
        ) -> Result<String> {
            let short = short.into();

            Ok(
                if short.is_empty() {
                    "No album found for specified user.".to_string()
                } else {
                    format!("https://catbox.moe/c/{short}")
                }
            )
        }

        pub async fn remove_files<S: Into<String>>(
            short: S,
            _user_hash: S,
            _files: Vec<S>
        ) -> Result<String> {
            let short = short.into();

            Ok(
                if short.is_empty() {
                    "No album found for specified user.".to_string()
                } else {
                    format!("https://catbox.moe/c/{short}")
                }
            )
        }
    }

    pub mod file {
        use anyhow::Result;
        use std::fs::File;
        use url::Url;

        pub async fn from_file<S: Into<String>>(file_path: S, _user_hash: S) -> Result<String> {
            let file_path = file_path.into();

            File::open(&file_path)?;

            Ok(format!("https://catbox.moe/file.{}", file_path.split('.').next_back().unwrap()))
        }

        pub async fn from_url<S: Into<String>>(url: S, _user_hash: S) -> Result<String> {
            let url = url.into();

            Url::parse(&url)?;

            Ok(format!("https://catbox.moe/file.{}", url.split('.').next_back().unwrap()))
        }

        pub async fn delete<S: Into<String>>(files: Vec<S>, _user_hash: S) -> Result<String> {
            let valid = files
                .into_iter()
                .map(Into::into)
                .all(|file| !file.is_empty());

            Ok(
                if valid {
                    "Files succesfully deleted.".to_string()
                } else {
                    "File doesn't exist?".to_string()
                }
            )
        }
    }

    pub mod litter {
        use anyhow::Result;
        use std::fs::File;

        pub async fn upload<S: Into<String>>(file_path: S, time: u8) -> Result<String> {
            let file_path = file_path.into();

            if ![1, 12, 24, 72].contains(&time) {
                return Ok("Invalid time".to_string());
            }

            File::open(&file_path)?;

            Ok(format!("https://catbox.moe/file.{}", file_path.split(',').next_back().unwrap()))
        }
    }
}
