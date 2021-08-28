use super::util::SortedVec;
use crate::error::{Error, Result};

use std::path::PathBuf;

use grep::{
    matcher::{Captures, Matcher},
    regex::RegexMatcher,
};

lazy_static::lazy_static! {
    pub static ref MATCHER: RegexMatcher = RegexMatcher::new_line_matcher(
        r"\{\{(?P<start>#|/)\s*lurien\s+(?P<hostname>[A-Za-z0-9\-_]+)\s*\}\}",
    )
    // Unwrap is safe as this regex is tested.
    .unwrap();
}

/// Represents a singular marker in a file. For example, `{{#lurien foo}}`. The struct is _stupid_;
/// it's just an intermediary step in creating a [`Snippet`](super::snippet::Snippet) and is not
/// stored anywhere.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Marker {
    /// The line number of the marker when all the other markers are in the file.
    pub lnum: u64,
    /// If the [`Marker`] is an opening marker (i.e. `{{#`) then this is true. Otherwise (i.e.
    /// closing marker i.e. `{{/`) it's set to false.
    pub is_opening: bool,
    /// The hostname specified in the marker.
    pub hostname: String,
}

impl Marker {
    pub fn with_line_number(self, lnum: u64) -> Self {
        Self { lnum, ..self }
    }
}

// Can't be made generic until specialisation is stabilised.
impl std::convert::TryFrom<&str> for Marker {
    type Error = Error;

    /// Tries to create a [`Marker`] (e.g. `{{#lurien foo}}` from a [`&str`]. Returns an
    /// `Err(`[`Error::NoMatch`]`)` if the input string does not match the needed regex
    /// pattern.
    fn try_from(value: &str) -> Result<Self> {
        // See comment below on unwrap.
        let mut captures = MATCHER.new_captures().unwrap();
        // `.captures` returns a Result<bool, ..>. However, an `Err` variant is not possible with
        // a `RegexMatcher` it only returns a Result because it has to implement the `Matcher` trait.
        // This means the unwrap is safe. The boolean specifies whether a match was found.
        let did_capture = MATCHER.captures(value.as_bytes(), &mut captures).unwrap();

        if !did_capture {
            return Err(Error::NoMatch);
        }

        // `captures.get(0)` returns the index of the entire match, `captures.get(1)` and
        // `captures.get(2)` return the indexes of captures group 1 and 2.
        let (start_match, hostname_match) = match (captures.get(1), captures.get(2)) {
            (Some(a), Some(b)) => (a, b),
            // This is unreachable as `MATCHER` will only match if both capture groups have
            // been matched.
            _ => unreachable!("not all regex capture groups have value"),
        };

        let is_opening = &value[start_match] == "#";
        let hostname = value[hostname_match].to_owned();

        Ok(Self {
            is_opening,
            lnum: 0,
            hostname,
        })
    }
}

/// Represents all the markers contained in a file.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FileMarkerData {
    pub path: PathBuf,
    /// `markers` being a [`SortedVec`] ensures that it contains the markers in the file from top
    /// to bottom.
    pub markers: SortedVec<Marker>,
}

impl std::convert::From<PathBuf> for FileMarkerData {
    fn from(path: PathBuf) -> Self {
        Self {
            path,
            markers: SortedVec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    fn gen_marker_str(is_opening: bool, hostname: &str, whitespace: &str) -> String {
        let mut result = String::new();

        result += "{{";
        result += if is_opening { "#" } else { "/" };
        result += whitespace;
        result += "lurien";
        result += " ";
        result += whitespace;
        result += hostname;
        result += whitespace;
        result += "}}";

        result
    }

    #[test]
    fn test_into_marker_ok() {
        let hostnames = vec![
            "james",
            "something-with-multiple-hypens",
            "we_allow_underscores_as_well",
            "and-even_a-mix",
            "but0it1looks2ugly",
            "_0-uoe4324hcaeu908",
        ];

        for hostname in hostnames {
            for is_opening in [true, false] {
                for whitespace in ["", " ", "  ", "     "] {
                    let input = gen_marker_str(is_opening, hostname, whitespace);
                    let expected = Marker {
                        is_opening,
                        lnum: 0,
                        hostname: hostname.to_owned(),
                    };
                    assert_eq!(Marker::try_from(input.as_ref()).unwrap(), expected);
                }
            }
        }
    }

    #[test]
    fn test_into_marker_err() {
        let tests = vec![
            "{#lurien singlebraces}",
            "{{ #lurien thing}}",
            // ^ illegal
            "{{#lurien name with space}}",
            //             ^----^-- illegal
            "{{#luriennospace}}",
        ];

        for input in tests {
            match Marker::try_from(input) {
                Err(Error::NoMatch) => {}
                _ => panic!(),
            }
        }
    }
}
