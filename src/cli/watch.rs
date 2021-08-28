use crate::error::Result;

use clap::Clap;

/// Watch for changes in the config and adjust the specified `.lurien` file accordingly
#[derive(Clap, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Watch {}

pub fn watch(_args: &Watch) -> Result<()> {
    todo!();
}
