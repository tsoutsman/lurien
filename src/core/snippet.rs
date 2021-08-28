use super::util::SortedVec;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Represents a snippet associated with a given hostname.
///
/// This struct is the final product of all
/// the processing that happens on a pair of opening and closing [`Markers`](super::marker::Marker).
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Snippet {
    /// The line number at which the snippet starts. This line number is calculated when the file
    /// has **no** snippets or markers in it.
    pub lnum: u64,
    pub hostname: String,
    pub content: String,
}

/// Represents all the snippets in a given file.
///
/// This is the struct that is _actually_ stored on
/// the disk, but it is simply a wrapper around snippets that allows each individual snippet to
/// not have to store its filepath. That would be quite inefficient as often files contain multiple
/// snippets.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct FileSnippetData {
    /// Relative path from the directory the `.lurien` file is stored in.
    pub path: PathBuf,
    /// `snippets` being a [`SortedVec`] ensures that it contains the snippets in the file from top
    /// to bottom. TODO at the moment the [`Ord`] and [`PartialOrd`] implementations first compare
    /// the `lnum`, and then `hostname` and `content`. Ideally we would want it to just compare
    /// `lnum`. So why don't you just manually implement the traits then? Because, [`PartialOrd`]
    /// uses the [`PartialEq`] implementation and so we would have to make [`PartialEq`] only
    /// compare `lnum` as well. I'm not sure this is what we want as it would make any two snippets
    /// _equal_ if they start on the same line, regardless of `hostname` or `content`.
    pub snippets: SortedVec<Snippet>,
}
