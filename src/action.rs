pub mod ls;

use crate::cli;
use crate::util::printer;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ActionErr {
    #[error("Cannot create action")]
    CreateErr,

    #[error("Unkown action type")]
    UnknownTypeErr,
}

pub trait Action<Out: std::io::Write, ErrOut: std::io::Write>: Sized {
    fn new(args: cli::Args, printer: printer::Printer<Out, ErrOut>) -> Result<Self, ActionErr>;

    fn run(&mut self);
}

pub fn create_action<Out: std::io::Write, ErrOut: std::io::Write>(
    args: cli::Args,
    printer: printer::Printer<Out, ErrOut>,
) -> Result<impl Action<Out, ErrOut>, ActionErr> {
    match args.action_type {
        ActionType::Ls => ls::LsAction::new(args, printer),
        _ => Err(ActionErr::UnknownTypeErr),
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

#[cfg(test)]
mod test_util {
    use super::*;
    use crate::cli::args;
    use crate::util::fs_util;
    use crate::util::printer;

    pub fn get_mock_action(
        action_type: ActionType,
    ) -> Result<impl Action<Vec<u8>, Vec<u8>>, ActionErr> {
        let printer = printer::test_util::get_mock_printer();
        let path = fs_util::test_util::get_mock_dir_path();
        let args = args::test_util::get_mock_args(action_type, Some(path));
        create_action(args, printer)
    }
}

#[cfg(test)]
mod tests {
    use super::test_util;
    use super::*;

    #[test]
    fn test_create_action() {
        let ls_action = test_util::get_mock_action(ActionType::Ls);
        assert_eq!(ls_action.is_ok(), true);
    }
}
