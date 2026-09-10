use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use std::str::FromStr;

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
    // /// Update dcr to the latest version
    // #[arg(short, long, exclusive = true)]
    // update: bool,
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

    /// Initialize the current directory as a project
    #[command(long_about = "Initializes the current directory as a DCR project.\n\
            The directory must be empty.")]
    Init {
        #[arg(long, value_enum)]
        vcs: Option<CliVCS>,
    },

    /// Remove the target directory
    #[command(long_about = "Removes build artifacts from the target directory.")]
    #[command(group(
        ArgGroup::new("mode")
            .args(["release", "debug", "all"])
    ))]
    Clean {
        /// Clean release artifacts
        #[arg(short, long)]
        release: bool,

        /// Clean debug artifacts (default)
        #[arg(short, long)]
        debug: bool,

        /// Clean artifacts for a specific target
        #[arg(short, long, exclusive = true,  value_parser = parse_target)]
        target: Option<String>,

        /// Clean all workspace members
        #[arg(short, long, exclusive = true)]
        all: bool,
    },
}

#[derive(ValueEnum, Debug, Clone)]
pub enum CliVCS {
    None,
    Git,
}

fn parse_target(raw_target: &str) -> Result<String, String> {
    let target = match raw_target {
        "" => "", // видимо норм, разобраться не баг ли
        "linux" => "x86_64-unknown-linux-gnu",
        "macos" => "x86_64-apple-darwin",
        "windows" => "x86_64-pc-windows-msvc",
        _ if raw_target.contains('-') => raw_target, // todo: правильная обработка ошибок формата
        _ => Err(format!(
            "Unknown target '{raw_target}', using as-is. Supported short names: linux, macos, windows",
        ))?, // возможно заменить на warn, посмотреть про поддержку сторонних компиляторов
    };

    Ok(target.to_string())
}

pub fn parse() -> CliArgs {
    CliArgs::parse()
}
