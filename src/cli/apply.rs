use crate::error::Result;

use clap::Clap;

/// Apply a particular configuration
#[derive(Clap, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Apply {}

pub fn apply(_args: &Apply) -> Result<()> {
    todo!();
}
