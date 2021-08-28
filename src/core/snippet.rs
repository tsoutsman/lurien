use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Represents a snippet associated with a given hostname. This struct is the final product of all
/// the processing that happens on a pair of opening and closing [`Markers`](super::marker::Marker).
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Snippet {
    /// The line number at which the snippet starts. This line number is calculated when the file
    /// has **no** snippets or markers in it.
    pub lnum: u64,
    pub hostname: String,
    pub content: String,
}

/// Represents all the snippets in a given file. This is the struct that is _actually_ stored on
/// the disk, but it is simply a wrapper around snippets that allows each individual snippet to
/// not have to store its filepath. That would be quite inefficient as often files contain multiple
/// snippets.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct FileSnippetData {
    /// Relative path from the directory the `.lurien` file is stored in.
    pub path: PathBuf,
    pub snippets: Vec<Snippet>,
}
