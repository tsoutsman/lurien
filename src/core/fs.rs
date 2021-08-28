//! This module contains all the functions that can be called by the cli subcommands.

use super::{
    marker::{FileMarkerData, Marker, MATCHER},
    snippet::FileSnippetData,
};
use crate::error::{Error, Result};

use std::{
    convert::TryFrom,
    io::{BufRead, BufReader, Write},
    path::Path,
};

use grep::searcher::Searcher;
use ignore::WalkBuilder;

/// Get all files containing markers, and the position of those markers, in a given directory.
/// The function will search the directory recursively.
pub fn markers<P: AsRef<Path>>(path: P) -> Result<Vec<FileMarkerData>> {
    // `.hidden(false)` means **include** hidden files.
    let walker = WalkBuilder::new(path).hidden(false).build();

    let mut searcher = Searcher::new();
    // TODO support names with weird characters and maybe spaces?
    let mut file_matches = Vec::new();

    for entry in walker {
        let entry = entry?;
        let mut file = FileMarkerData::from(entry.into_path());

        // TODO this will not be necessary in Rust 2021 when closures can partially move structs.
        let path = file.path.clone();

        searcher.search_path(
            &*MATCHER,
            path,
            // TODO this sink or lossy?
            grep::searcher::sinks::UTF8(|lnum, line| {
                // Unwrap is safe as the line is guaranteed to contain a match.
                file.markers
                    // TODO gracefully exit instead of unwrapping.
                    .try_push(Marker::try_from(line).unwrap().with_line_number(lnum))
                    .unwrap();
                Ok(true)
            }),
        )?;

        file_matches.push(file);
    }

    Ok(file_matches)
}

/// Remove all the markers specified in a [`Vec`] of [`FileMarkerData`].
pub fn remove_markers(files: Vec<FileMarkerData>) -> Result<()> {
    for marked_file in files {
        let mut result = String::new();
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(marked_file.path)?;

        let mut lines = BufReader::new(&file).lines().enumerate();
        let mut lnum = 0;

        for marker in marked_file.markers {
            while lnum < marker.lnum {
                let next = match lines.next() {
                    Some((lnum, line)) => (lnum, line?),
                    None => return Err(Error::ExpectedMarker),
                };

                lnum = next.0 as u64;
                let line_content = next.1;

                result.push_str(&line_content);
            }
        }

        file.write_all(result.as_bytes())?;
    }

    Ok(())
}

pub fn process_markers(marked_files: Vec<FileMarkerData>) -> Result<Vec<FileSnippetData>> {
    for _file in marked_files {
        //
    }
    todo!();
}
