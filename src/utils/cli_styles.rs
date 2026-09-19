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

pub use owo_colors::{OwoColorize, Style};

// // #[allow(dead_code)]
// pub const BRIGHT_RED: Style = Style::new().bright_red();
// // #[allow(dead_code)]
// pub const BRIGHT_GREEN: Style = Style::new().bright_green();
// // #[allow(dead_code)]
// pub const BRIGHT_YELLOW: Style = Style::new().bright_yellow();
// // #[allow(dead_code)]
// pub const BRIGHT_CYAN: Style = Style::new().bright_cyan();
// #[allow(dead_code)]
pub const BOLD_RED: Style = Style::new().bright_red().bold();
// #[allow(dead_code)]
pub const BOLD_GREEN: Style = Style::new().bright_green().bold();
// #[allow(dead_code)]
pub const BOLD_YELLOW: Style = Style::new().bright_yellow().bold();
// #[allow(dead_code)]
pub const BOLD_CYAN: Style = Style::new().bright_cyan().bold();
// #[allow(dead_code)]
pub const BOLD_BLUE: Style = Style::new().bright_blue().bold();

// /// Applies ANSI escape codes to format the message with the given style.
// ///
// /// This is a utility for colored output in terminal applications.
// #[allow(dead_code)]
// pub fn styled(msg: &str, style: Style) -> String {
//     msg.style(style).to_string()
// }

/// Prints `msg` to stdout with the given `style`.
#[allow(dead_code)]
pub fn printc(msg: &str, style: Style) {
    println!("{}", msg.style(style));
}

// #[macro_export]
// macro_rules! s_println {
//     ($style:expr, $($arg:tt)*) => {
//         {
//             use ::owo_colors::OwoColorize as _;
//             println!("{}", format!($($arg)*).style($style));
//         }
//     };
// }

// #[macro_export]
// macro_rules! s_print {
//     ($style:expr, $($arg:tt)*) => {
//         {
//             use ::owo_colors::OwoColorize as _;
//             println!("{}", format!($($arg)*).style($style));
//         }
//     };
// }

// // #[allow(dead_code)]
// pub use crate::{s_print, s_println};
