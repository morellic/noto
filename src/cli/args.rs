pub use structopt::StructOpt;

use crate::action::ActionType;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(possible_values = &ActionType::variants(), case_insensitive = true)]
    pub action_type: ActionType,

    #[structopt(parse(from_os_str))]
    pub path: Option<std::path::PathBuf>,
}

#[cfg(test)]
pub mod test_util {
    use super::ActionType;
    use super::Args;

    pub fn get_mock_args(action_type: ActionType, path: Option<std::path::PathBuf>) -> Args {
        Args { action_type, path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actions() {
        _test_action("find", ActionType::Find);
        _test_action("grep", ActionType::Grep);
        _test_action("ls", ActionType::Ls);
    }

    fn _test_action(action_str: &str, action_type: ActionType) {
        _test_no_action();
        _test_action_with_path(action_str, action_type.clone());
        _test_action_without_path(action_str, action_type.clone());
    }

    fn _test_no_action() {
        let args = Args::from_iter_safe(&["bin"]);
        assert_eq!(args.is_err(), true);
    }

    fn _test_action_with_path(action_str: &str, action_type: ActionType) {
        let args: Args = Args::from_iter(&["bin", action_str, "some-dir"]);
        assert_eq!(args.action_type, action_type);
        assert_eq!(args.path.is_some(), true);
        assert_eq!(args.path.unwrap().to_str().unwrap(), "some-dir");
    }

    fn _test_action_without_path(action_str: &str, action_type: ActionType) {
        let args: Args = Args::from_iter(&["bin", action_str]);
        assert_eq!(args.action_type, action_type);
        assert_eq!(args.path.is_none(), true);
    }
}
