//! With this module you can perform actions that are input related.
//! Like reading a line, reading a character and reading asynchronously.

use std::io;
use std::sync::Arc;
use super::*;
use modules::terminal::*;

pub type Terminal = Box<ITerminal>;

pub fn input() -> Terminal {
    #[cfg(target_os = "windows")]
    let terminal = functions::get_module::<Box<ITerminal>>(
        Box::new(WinApiTerminal::new()),
        Box::new(AnsiTerminal::new()),
    ).unwrap();

    #[cfg(not(target_os = "windows"))]
    let terminal = Box::from(AnsiTerminal::new()) as Box<ITerminal>;

    terminal
}

