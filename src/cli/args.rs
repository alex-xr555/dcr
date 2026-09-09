use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    version = concat!(
        env!("CARGO_PKG_VERSION"),
        env!("DCR_GIT_INFO"),
        " (", env!("DCR_TARGET"), ")"
    ),
    about = r#"DCR (Dexoron Cargo Realization)
C project manager inspired by Cargo."#,
    long_about = None,
    arg_required_else_help = true
)]
pub struct CliArgs {
    #[command(subcommand)]
    pub mode: CliMode,
}

#[derive(Subcommand, Debug)]
pub enum CliMode {
    /// Create a new project
    #[command(long_about = "Creates a new C/C++ project with the given name.\n\
            The name may only contain ASCII letters, digits, '_' and '-'.")]
    New {
        name: String,

        #[arg(long, value_enum)]
        vcs: Option<CliVCS>,
    },
    // /// Initialize the current directory as a project
    // Init,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum CliVCS {
    None,
    Git,
}

pub fn parse() -> CliArgs {
    CliArgs::parse()
}
