// DCR — Cargo-like C/C++ project manager.
//
// Copyright (C) 2026 Dexoron (Bezotechestvo Vladimir) <main@dexoron.su>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use crate::utils::log::error;
mod cli;
mod config;
mod core;
mod platform;
mod utils;

use crate::cli::args::CliMode::*;

/// Main entry point for DCR, a Cargo-like C/C++ project manager.
///
/// Collects command-line arguments via `std::env::args()` and dispatches to the
/// appropriate subcommand handler. With fewer than two arguments, shows help and
/// exits with code 0; otherwise exits with the handler's exit code.
fn main() {
    let args = cli::args::parse();
    dbg!(&args);

    let code = match args.mode {
        New { name, vcs } => cli::new::new(&name, &vcs),
        Init { vcs } => cli::init::init(&vcs),
        _ => todo!("режим не готов"),
    };

    std::process::exit(code);

    //     "setup" => cli::setup::setup(rest),
    //     "add" => cli::add::add(rest),
    //     "build" => cli::build::build(rest),
    //     "run" => cli::run::run(rest),
    //     "tree" => cli::tree::tree(rest),
    //     "test" | "tests" => cli::test::test(rest),
    //     "fmt" => cli::fmt::fmt(rest),
    //     "lint" => cli::lint::lint(rest),
    //     "clean" => cli::clean::clean(rest),
    //     "gen" => cli::r#gen::r#gen(rest),
    //     "--update" => cli::flag_update::flag_update(rest),
}
