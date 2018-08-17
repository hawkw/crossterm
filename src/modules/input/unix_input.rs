//! This is an UNIX specific implementation for input related action.

use std::char;
use std::io::{self, Read, Write};
use std::sync::{mpsc, Arc};
use std::thread;

use super::{AsyncReader, ITerminalInput};
use kernel::unix_kernel::terminal::{get_tty, read_char};

pub struct UnixInput;

impl UnixInput {
    pub fn new() -> UnixInput {
        UnixInput
    }
}

impl ITerminalInput for UnixInput {
    fn read_line(&self, _: bool) -> io::Result<String> {
        let mut rv = String::new();
        io::stdin().read_line(&mut rv)?;
        let len = rv.trim_right_matches(&['\r', '\n'][..]).len();
        rv.truncate(len);
        Ok(rv)
    }

    fn read_char(&self, _: bool) -> io::Result<char> {
        read_char()
    }

    fn read_async(&self, _: bool) -> AsyncReader {
        let (send, recv) = mpsc::channel();

        thread::spawn(move || {
            for i in get_tty().unwrap().bytes() {
                if send.send(i).is_err() {
                    return;
                }
            }
        });

        AsyncReader { recv: recv }
    }

    fn read_until_async(&self, delimiter: u8, _: bool) -> AsyncReader {
        let (send, recv) = mpsc::channel();

        thread::spawn(move || {
            for i in get_tty().unwrap().bytes() {
                match i {
                    Ok(byte) => {
                        let end_of_stream = &byte == &delimiter;
                        let send_error = send.send(Ok(byte)).is_err();

                        if end_of_stream || send_error {
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });

        AsyncReader { recv: recv }
    }
}
