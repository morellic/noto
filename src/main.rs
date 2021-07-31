mod action;
mod cli;
mod util;

use structopt::StructOpt;

fn main() {
    let mut w = std::io::stdout();
    let args = cli::Args::from_args();
    let path = get_path(&args, &mut w);
    match args.action_type {
        action::ActionType::Find => (), // TODO: impl action::find
        action::ActionType::Grep => (), // TODO: impl action::grep
        action::ActionType::Ls => action::ls(&path, &mut w),
    }
}

fn get_path(args: &cli::Args, w: &mut impl std::io::Write) -> std::path::PathBuf {
    if args.path.is_some() {
        return args.path.clone().unwrap();
    }
    util::fs::get_current_dir(w).unwrap()
}
