use crate::error::Result;

use clap::Clap;

/// Save the markers in the specified directory into a `.lurien` file
#[derive(Clap, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Save {}

pub fn save(_args: &Save) -> Result<()> {
    todo!();
}
