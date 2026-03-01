use std::{fmt::Display, path::PathBuf};

use tokio::{fs::File, io::AsyncWriteExt};

pub const DXVK_SOURCE: &str =
    "https://github.com/doitsujin/dxvk/releases/download/v2.7.1/dxvk-2.7.1.tar.gz";
pub const VKD3D_PROTON_SOURCE: &str = "https://github.com/HansKristian-Work/vkd3d-proton/releases/download/v3.0b/vkd3d-proton-3.0b.tar.zst";
pub const WINE_RUNNER_SOURCE: &str = "https://dawn.wine/NelloKudo/wine-miniloader/releases/download/wine-cachyos-miniloader/wine-cachyos-miniloader-fonts-10.0-1-x86_64.tar.xz";

pub struct Client {
    pub client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Client {
    pub async fn download<F>(
        &self,
        url: &str,
        destination: PathBuf,
        message: &str,
        callback: F,
    ) -> Result<PathBuf, DownloadError>
    where
        F: Fn(DownloadCallback),
    {
        let request = self.client.get(url).send().await;
        if request.is_err() {
            return Err(DownloadError::UnableToDownload(message.into()));
        }

        let mut request = request.unwrap();
        let request_length = request.content_length().unwrap_or(0);

        let mut downloaded: u64 = 0;
        let file = File::create(&destination).await;
        if file.is_err() {
            return Err(DownloadError::UnableToDownload(message.into()));
        }

        let mut file = file.unwrap();

        while let Some(chunk) = request
            .chunk()
            .await
            .map_err(|_| DownloadError::UnableToDownload(message.into()))?
        {
            let chunk_len = chunk.len();

            let is_chunk_empty = chunk_len == 0;
            if is_chunk_empty {
                continue;
            }

            let written = file.write_all(&chunk).await;
            if written.is_err() {
                return Err(DownloadError::UnableToDownload(message.into()));
            }

            downloaded += chunk_len as u64;
            let percentage = if request_length > 0 {
                (downloaded as f64 / request_length as f64 * 100.0) as u64
            } else {
                0
            };

            callback(DownloadCallback {
                message: message.into(),
                total: request_length,
                downloaded,
                percentage,
            })
        }

        Ok(destination.into())
    }
}

#[derive(Debug, Clone)]
pub enum DownloadError {
    UnableToDownload(String),
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadError::UnableToDownload(message) => write!(f, "Unable to download {}", message),
        }
    }
}

pub struct DownloadCallback {
    pub message: String,

    pub total: u64,
    pub percentage: u64,
    pub downloaded: u64,
}
