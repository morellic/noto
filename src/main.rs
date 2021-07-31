mod action;
mod cli;
mod util;

use action::ActionType;
use cli::Args;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();
    let mut w = std::io::stdout();
    match args.action_type {
        ActionType::Find => (), // TODO: impl action::find
        ActionType::Grep => (), // TODO: impl action::grep
        ActionType::Ls => action::ls(args.path, &mut w),
    }
}
