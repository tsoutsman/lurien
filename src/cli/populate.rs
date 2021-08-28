use crate::error::Result;

use clap::Clap;

/// Apply the markers from a `.lurien` file
#[derive(Clap, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Populate {}

pub fn populate(_args: &Populate) -> Result<()> {
    todo!();
}
