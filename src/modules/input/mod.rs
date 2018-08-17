//! With this module you can perform actions that are input related.
//! Like reading a line, reading a character and reading asynchronously.


#[cfg(not(target_os = "windows"))]
mod unix_input;
#[cfg(target_os = "windows")]
mod windows_input;

#[cfg(not(target_os = "windows"))]
use self::unix_input::UnixInput;
#[cfg(target_os = "windows")]
use self::windows_input::WindowsInput;


use std::io::{self, Error, ErrorKind, Read};
use std::sync::{mpsc, Arc};

/// This trait defines the actions that can be preformed with the terminal color.
/// This trait can be implemented so that an concrete implementation of the ITerminalColor can forfill
/// the wishes to work on an specific platform.
///
/// ## For example:
///
/// This trait is implemented for Windows and UNIX systems.
/// Unix is using the tty and windows is using libc C functions to read the input.
trait ITerminalInput {
    /// Read one line from the user input
    fn read_line(&self, raw_mode: bool) -> io::Result<String>;
    /// Read one character from the user input
    fn read_char(&self, raw_mode: bool) -> io::Result<char>;
    /// Read the input asynchronously from the user.
    fn read_async(&self, raw_mode: bool) -> AsyncReader;
    ///  Read the input asynchronously until a certain character is hit.
    fn read_until_async(&self, delimiter: u8, raw_mode: bool) -> AsyncReader;
}

/// This is a wrapper for reading from the input asynchronously.
/// This wrapper has a channel receiver that receives the input from the user whenever it typed something.
/// You only need to check whether there are new characters available.
pub struct AsyncReader {
    recv: mpsc::Receiver<io::Result<u8>>,
}

impl Read for AsyncReader {
    /// Read from the byte stream.
    ///
    /// This will never block, but try to drain the event queue until empty. If the total number of
    /// bytes written is lower than the buffer's length, the event queue is empty or that the event
    /// stream halted.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut total = 0;

        loop {
            if total >= buf.len() {
                break;
            }

            match self.recv.try_iter().next() {
                Some(Ok(value)) => {
                    buf[total] = value;
                    total += 1;
                }
                _ => return Err(Error::new(ErrorKind::Other, "No characters pressed.")),
            }
        }

        Ok(total)
    }
}
