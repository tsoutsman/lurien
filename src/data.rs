use crate::error::Result;

use std::{
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Snippet {
    /// The line number at which the snippet starts.
    pub lnum: u64,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct File {
    /// Relative path from the directory the `.lurien` file is stored in.
    pub path: PathBuf,
    pub snippets: Vec<Snippet>,
}

pub fn store_data(files: &[File], location: &Path) -> Result<()> {
    let file_path = location.join(".lurien");
    let data_file = std::fs::File::create(file_path)?;
    let mut writer = BufWriter::new(data_file);

    for file in files {
        bincode::serialize_into(&mut writer, file)?;
    }

    writer.flush()?;
    Ok(())
}

pub fn read_data(path: &Path) -> Result<Vec<File>> {
    let mut reader = BufReader::new(std::fs::File::open(path)?);
    Ok(bincode::deserialize_from(&mut reader)?)
}
