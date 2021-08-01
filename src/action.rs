pub mod ls;

use crate::util::printer;

pub struct ActionParams {
    pub action_type: ActionType,
    pub path: std::path::PathBuf,
    pub printer: printer::Printer<std::io::Stdout, std::io::Stderr>,
}

pub fn run_action(params: ActionParams) {
    match params.action_type {
        ActionType::Find => (), // TODO: impl find
        ActionType::Grep => (), // TODO: impl grep
        ActionType::Ls => ls::ls(params),
    }
}

use structopt::clap::arg_enum;
arg_enum! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ActionType {
        Find,
        Grep,
        Ls,
    }
}
