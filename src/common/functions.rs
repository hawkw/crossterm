//! Some actions need to preformed platform independently since they can not be solved `ANSI escape codes`.

use std::sync::Arc;

#[cfg(windows)]
use kernel::windows_kernel::ansi_support::{try_enable_ansi_support, windows_supportable};

#[cfg(windows)]
use kernel::windows_kernel::terminal::{buffer_size, exit, terminal_size};

#[cfg(windows)]
use kernel::windows_kernel::cursor::pos;

#[cfg(unix)]
use kernel::unix_kernel::terminal::{exit, pos, terminal_size};

/// Get the terminal size based on the current platform.
pub fn get_terminal_size() -> (u16, u16) {
    return terminal_size();
}

/// Get the cursor position based on the current platform.
pub fn get_cursor_position(stdout: Arc<TerminalOutput>, raw_mode: bool) -> (u16, u16) {
    return pos(stdout, raw_mode);
}

/// exit the current terminal.
pub fn exit_terminal() {
    exit();
}

#[cfg(windows)]
/// Get an module specific implementation of a the generic given type based on the current platform.
/// If the current platform is windows and it supports ansi escape codes it will return the ansi implementation and if not it will return the winapi implementation.
/// If the current platform is unix it will return the ansi implementation.
pub fn get_module<T>(winapi_impl: T, unix_impl: T) -> Option<T> {
    let mut term: Option<T> = None;
    let mut does_support = true;

    if !windows_supportable() {
        // Try to enable ansi on windows if not than use WINAPI.
        does_support = try_enable_ansi_support();

        // uncomment this line when you want to use the winapi implementation.
        does_support = false;
        if !does_support {
            term = Some(winapi_impl);
        }
    }

    if does_support {
        term = Some(unix_impl);
    }

    term
}
