#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use colored::{Colorize, ColoredString};
use clap::Parser;
use crate::cli;

#[derive(PartialEq, Eq)]
pub enum LogMode {
    Info,
    Error,
    Warning,
    Note,
    Debug,
    Todo,
}

macro_rules! generic {
    ($($arg:tt)*) => ({
        log_generic_print(format!($($arg)*));
    });
}

macro_rules! info {
    ($($arg:tt)*) => ({
        log_core_print(format!($($arg)*), LogMode::Info);
    });
}

macro_rules! error {
    ($($arg:tt)*) => ({
        log_core_print(format!($($arg)*), LogMode::Error);
    });
}

macro_rules! warning {
    ($($arg:tt)*) => ({
        log_core_print(format!($($arg)*), LogMode::Warning);
    });
}

macro_rules! note {
    ($($arg:tt)*) => ({
        log_core_print(format!($($arg)*), LogMode::Note);
    });
}

macro_rules! debug {
    ($($arg:tt)*) => ({
        log_core_print(format!($($arg)*), LogMode::Debug);
    });
}

macro_rules! task {
    ($($arg:tt)*) => ({
        log_core_print(format!($($arg)*), LogMode::Todo);
    });
}

pub fn log_generic_print(msg: String) {
    println!("{} {}", " :".black().bold(), msg);
}

pub fn log_core_print(msg: String, mode: LogMode) {
    let args = cli::Cli::parse();

    if args.debug == false && mode == LogMode::Debug {
        return;
    }

    let prefix_text: &str = match mode {
        LogMode::Info => "Info",
        LogMode::Error => "Error",
        LogMode::Warning => "Warning",
        LogMode::Note => "Note",
        LogMode::Debug => "Debug",
        LogMode::Todo => "TODO",
    };

    let prefix = apply_color(prefix_text.to_string(), &mode);

    println!("{left}{}{right} {}", prefix.bold(), msg, left = "[".black().bold(), right = "] :".black().bold());
}

fn apply_color(string: String, mode: &LogMode) -> String {
    let colored_string: ColoredString = match mode {
        LogMode::Info => string.bright_green(),
        LogMode::Error => string.bright_red(),
        LogMode::Warning => string.bright_yellow(),
        LogMode::Note => string.bright_yellow(),
        LogMode::Debug => string.bright_magenta(),
        LogMode::Todo => string.bright_cyan(),
    };

    return colored_string.to_string();
}

pub(crate) use generic;
pub(crate) use info;
pub(crate) use error;
pub(crate) use warning;
pub(crate) use note;
pub(crate) use debug;
pub(crate) use task;

pub mod macro_deps {
    pub use super::{LogMode, log_generic_print, log_core_print};
}
