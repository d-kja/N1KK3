use std::{fmt::Display, fs::File, path::PathBuf};

use flate2::read::GzDecoder;
use liblzma::read::XzDecoder;
use tar::Archive;
use tokio::task;

pub struct Extract {}

#[derive(Debug, Clone)]
pub enum ExtractError {
    UnableToExtractFile(String),
    ExtensionNotFound(String),
    UnknownFileFormat(String),
    FileNameNotFound(String),
    UnableToReadArchive(String),
    FailedExtractTask,
}

impl Display for ExtractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractError::UnableToExtractFile(file) => {
                write!(f, "Unable to extract archive: {}", file)
            }
            ExtractError::ExtensionNotFound(file) => {
                write!(f, "Extension not found for archive: {}", file)
            }
            ExtractError::FileNameNotFound(file) => {
                write!(f, "File name not found for archive: {}", file)
            }
            ExtractError::UnknownFileFormat(file) => {
                write!(f, "File {} has an unknown file format", file)
            }
            ExtractError::UnableToReadArchive(file) => {
                write!(f, "Unable to read archive {}", file)
            }
            ExtractError::FailedExtractTask => {
                write!(f, "Unable to extract archive, task failed")
            }
        }
    }
}

impl Extract {
    pub async fn extract(file_path: PathBuf, destination: PathBuf) -> Result<(), ExtractError> {
        let archive_task = task::spawn_blocking(move || {
            let file_name = file_path
                .file_name()
                .expect("Archive should have a name")
                .to_str();
            if file_name.is_none() {
                return Err(ExtractError::ExtensionNotFound("unknown".into()));
            }

            let file_name = file_name.unwrap();

            let ext = file_path.extension();
            if ext.is_none() {
                return Err(ExtractError::UnableToExtractFile(file_name.into()));
            }

            let ext = ext.unwrap().to_str();

            let file = File::open(&file_path);
            if file.is_err() {
                return Err(ExtractError::UnableToReadArchive(file_name.into()));
            }

            let file = file.unwrap();

            match ext {
                Some("gz") => {
                    let decoder = GzDecoder::new(file);

                    let result = Archive::new(decoder).unpack(&destination);
                    if result.is_err() {
                        return Err(ExtractError::UnableToExtractFile(file_name.into()));
                    }

                    return Ok(());
                }
                Some("zst") => {
                    let decoder = zstd::Decoder::new(file);
                    if decoder.is_err() {
                        return Err(ExtractError::UnableToExtractFile(file_name.into()));
                    }

                    let decoder = decoder.unwrap();

                    let result = Archive::new(decoder).unpack(&destination);
                    if result.is_err() {
                        return Err(ExtractError::UnableToExtractFile(file_name.into()));
                    }

                    return Ok(());
                }
                Some("xz") => {
                    let decoder = XzDecoder::new(file);

                    let result = Archive::new(decoder).unpack(&destination);
                    if result.is_err() {
                        return Err(ExtractError::UnableToExtractFile(file_name.into()));
                    }

                    return Ok(());
                }
                _ => return Err(ExtractError::UnknownFileFormat(file_name.into())),
            }
        })
        .await;

        if archive_task.is_err() {
            return Err(ExtractError::FailedExtractTask);
        }

        archive_task.unwrap()
    }
}
