use crate::error::Result;

use clap::Clap;

/// Place all markers specified in a `.lurien` file into their positions
#[derive(Clap, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Populate {}

pub fn populate(_args: &Populate) -> Result<()> {
    todo!();
}
