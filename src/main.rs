mod action;
mod cli;
mod util;

use structopt::StructOpt;

fn main() {
    let args = cli::Args::from_args();

    let mut printer = util::printer::Printer {
        out: std::io::stdout(),
        err_out: std::io::stderr(),
    };

    if let Some(path) = get_path(&args, &mut printer) {
        action::run_action(action::ActionParams {
            action_type: args.action_type,
            path,
            printer,
        });
    }
}

fn get_path<Out: std::io::Write, ErrOut: std::io::Write>(
    args: &cli::Args,
    mut printer: &mut util::printer::Printer<Out, ErrOut>,
) -> Option<std::path::PathBuf> {
    if args.path.is_some() {
        return args.path.clone();
    }
    util::fs::get_current_dir(&mut printer)
}
