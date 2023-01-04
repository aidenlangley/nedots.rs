use clap::Parser;
use nedots::{Execute, RootCmd};
use std::process::ExitCode;

fn init() -> RootCmd {
    let root_cmd = RootCmd::parse();

    // Set logging level to the given verbosity, but if it's set lower than
    // error, then bump it up to info - unless user has explicitly asked for
    // silence, in that case we'll leave it be.
    let mut verbosity = root_cmd.verbose.log_level_filter();
    if root_cmd
        .verbose
        .log_level_filter()
        .le(&log::LevelFilter::Error)
        && !root_cmd.verbose.is_silent()
    {
        verbosity = log::LevelFilter::Info;
    }

    // Initialize our logger
    if cfg!(debug_assertions) {
        // If debugging, leave timestamp & target
        env_logger::Builder::new().filter_level(verbosity).init();
    } else {
        // User will see this output, so we'll make it a little more friendly
        env_logger::Builder::new()
            .format_timestamp(None)
            .format_target(false)
            .filter_level(verbosity)
            .init();
    }

    log::debug!("{:#?}", root_cmd);
    root_cmd
}

fn run(root_cmd: RootCmd) -> ExitCode {
    match root_cmd.exec(&root_cmd) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            log::error!("❌ {}", err);
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    run(init())
}
