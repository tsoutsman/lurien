use clap::Clap;

#[derive(Clap, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[clap(
    name = "lurien",
    version = "0.1",
    author = "Klim Tsoutsman",
    rename_all = "kebab"
)]
pub enum App {
    Populate(Populate),
    Save(Save),
    Watch(Watch),
}

/// Apply the markers from a `.lurien` file
#[derive(Clap, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Populate {}

/// Save the markers in the specified directory into a `.lurien` file
#[derive(Clap, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Save {}

/// Watch for changes in the config and adjust the specified `.lurien` file accordingly
#[derive(Clap, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Watch {}
