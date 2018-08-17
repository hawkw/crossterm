//! With this module you can perform actions that are cursor related.
//! Like moving the cursor position;saving and resetting the cursor position; hiding showing and control the blinking of the cursor.

mod ansi_cursor;
#[cfg(target_os = "windows")]
mod winapi_cursor;

pub use self::ansi_cursor::AnsiCursor;
#[cfg(target_os = "windows")]
pub use self::winapi_cursor::WinApiCursor;

use super::{functions};
use TerminalOutput;

use std::sync::Arc;

///! This trait defines the actions that can be preformed with the terminal cursor.
///! This trait can be implemented so that an concrete implementation of the ITerminalCursor can forfill
///! the wishes to work on an specific platform.
///!
///! ## For example:
///!
///! This trait is implemented for `WINAPI` (Windows specific) and `ANSI` (Unix specific),
///! so that cursor related actions can be preformed on both unix and windows systems.
trait ITerminalCursor: Send+Sync {
    /// Goto some location (x,y) in the context.
    fn goto(&self, x: u16, y: u16, terminal_output: &Arc<TerminalOutput>);
    /// Get the location (x,y) of the current cusror in the context
    fn pos(&self, terminal_output: &Arc<TerminalOutput>, raw_mode: bool) -> (u16, u16);
    /// Move cursor n times up
    fn move_up(&self, count: u16, terminal_output: &Arc<TerminalOutput>);
    /// Move the cursor `n` times to the right.
    fn move_right(&self, count: u16, terminal_output: &Arc<TerminalOutput>);
    /// Move the cursor `n` times down.
    fn move_down(&self, count: u16, terminal_output: &Arc<TerminalOutput>);
    /// Move the cursor `n` times left.
    fn move_left(&self, count: u16, terminal_output: &Arc<TerminalOutput>);
    /// Save cursor position so that its saved position can be recalled later. Note that this position is stored program based not per instance of the cursor struct.
    fn save_position(&self, terminal_output: &Arc<TerminalOutput>);
    /// Return to saved cursor position
    fn reset_position(&self, terminal_output: &Arc<TerminalOutput>);
    /// Hide the terminal cursor.
    fn hide(&self, terminal_output: &Arc<TerminalOutput>);
    /// Show the terminal cursor
    fn show(&self, terminal_output: &Arc<TerminalOutput>);
    /// Enable or disable the blinking of the cursor.
    fn blink(&self, blink: bool, terminal_output: &Arc<TerminalOutput>);
}
