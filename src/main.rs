mod action;
mod cli;
mod util;

use action::Action;
use structopt::StructOpt;

fn run_cli(args: cli::Args) -> Result<(), ()> {
    let printer = util::printer::Printer {
        out: std::io::stdout(),
        err_out: std::io::stderr(),
    };

    match action::create_action(args, printer) {
        Ok(mut action) => {
            action.run();
            Ok(())
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(())
        }
    }
}

fn main() {
    let args = cli::Args::from_args();
    std::process::exit(match run_cli(args) {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
