use super::Run;
use crate::{models::config::Config, RootCmd};
use clap_complete::{generate, shells};

#[derive(Debug, clap::Args)]
/// Generate completions
pub(crate) struct CompletionsCmd {
    shell: shells::Shell,
}

impl super::Run for CompletionsCmd {
    fn run(&self) -> anyhow::Result<()> {
        generate(
            self.shell,
            &mut <crate::RootCmd as clap::CommandFactory>::command(),
            "nedots",
            &mut std::io::stdout(),
        );
        Ok(())
    }
}
