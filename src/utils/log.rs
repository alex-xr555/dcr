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

use anstyle::{AnsiColor, Style};
use env_logger::*;
use log::*;
use std::io::Write;

// Функция инициализирующая логирование
pub fn init() {
    Builder::new()
        .filter_level(LevelFilter::Warn)
        .format(|buf, record| {
            let color = match record.level() {
                Level::Error => AnsiColor::Red,
                Level::Warn => AnsiColor::Yellow,
                Level::Info => AnsiColor::White,
                Level::Debug => AnsiColor::Magenta,
                Level::Trace => AnsiColor::BrightBlack,
            };

            let style = Style::new().fg_color(Some(color.into())).bold();

            writeln!(
                buf,
                "{style}{}{style:#}: {}",
                record.level().to_string().to_lowercase(),
                record.args(),
            )
        })
        .init();

    // env_logger::init(); // Уровень логирования из переменных окружения
}
