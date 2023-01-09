use clap_complete::{generate, shells};

#[derive(Debug, clap::Args)]
pub struct CompletionsCmd {
    /// The shell to generate completions for
    shell: shells::Shell,
}

impl super::Run for CompletionsCmd {
    fn run(&self) -> anyhow::Result<()> {
        generate(
            self.shell,
            &mut <super::nedots::RootCmd as clap::CommandFactory>::command(),
            "nedots",
            &mut std::io::stdout(),
        );
        Ok(())
    }
}
