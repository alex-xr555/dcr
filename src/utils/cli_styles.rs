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

// Сli theme styles
// #[allow(dead_code)]
pub const BOLD_RED: Style = Style::new().bright_red().bold();
// #[allow(dead_code)]
pub const BOLD_GREEN: Style = Style::new().bright_green().bold(); //BOLD_GREEN
// #[allow(dead_code)]
pub const BOLD_YELLOW: Style = Style::new().bright_yellow().bold();
// #[allow(dead_code)]
pub const BOLD_CYAN: Style = Style::new().bright_cyan().bold();
// #[allow(dead_code)]
pub const BOLD_BLUE: Style = Style::new().bright_blue().bold();

// #[allow(dead_code)]
pub const ERROR_ST: Style = BOLD_RED;
// #[allow(dead_code)]
pub const SUCCESS_ST: Style = BOLD_GREEN;
// #[allow(dead_code)]
pub const ALERT_ST: Style = BOLD_YELLOW;
// #[allow(dead_code)]
pub const SKIP_ST: Style = BOLD_BLUE;
