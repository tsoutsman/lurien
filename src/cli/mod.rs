mod apply;
mod populate;
mod save;
mod watch;

pub use apply::apply;
pub use populate::populate;
pub use save::save;
pub use watch::watch;

use apply::Apply;
use clap::Clap;
use populate::Populate;
use save::Save;
use watch::Watch;

#[derive(Clap, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[clap(
    name = "lurien",
    version = "0.1",
    author = "Klim Tsoutsman",
    rename_all = "kebab"
)]
pub enum App {
    Apply(Apply),
    Populate(Populate),
    Save(Save),
    Watch(Watch),
}
