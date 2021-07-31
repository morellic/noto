pub mod ls;

pub use ls::ls;

use structopt::clap::arg_enum;

arg_enum! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ActionType {
        Find,
        Grep,
        Ls,
    }
}
