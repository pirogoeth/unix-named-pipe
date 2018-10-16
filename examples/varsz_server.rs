//! This is a sample named pipe server.
//! The server expects a file path to be passed on the command line as the first arg.
//! The server will create a pipe at the given path if it doesn't already exist.
//! The server will read pairs of bytes at a time and print the randomly generated number
//! to stdout.

extern crate ctrlc;
#[macro_use]
extern crate miniserde;
extern crate unix_named_pipe;

use miniserde::json;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use unix_named_pipe::FileFIFOExt;

#[derive(Debug, MiniDeserialize)]
struct Message {
    numbers: Vec<u8>,
}

fn main() {
    let pipe_path = env::args()
        .nth(1)
        .expect("named pipe path required but not provided");
    println!("server opening pipe: {}", pipe_path);

    // Set up a keyboard interrupt handler so we can remove the pipe when
    // the process is shut down.
    let running = make_loop_flag();

    // Open the pipe file for reading
    let file = try_open(&pipe_path).expect("could not open pipe for reading");
    let mut reader = io::BufReader::new(file);

    // Loop reading from the pipe until a keyboard interrupt is received
    while running.load(Ordering::SeqCst) {
        // If an error occurs during read, panic
        let mut line = String::new();
        let res = reader.read_line(&mut line);
        if let Err(err) = res {
            // Named pipes, by design, only support nonblocking reads and writes.
            // If a read would block, an error is thrown, but we can safely ignore it.
            match err.kind() {
                io::ErrorKind::WouldBlock => continue,
                _ => panic!(format!("error while reading from pipe: {:?}", err)),
            }
        } else if let Ok(count) = res {
            if count == 0 {
                continue;
            } else {
                let payload: Message = json::from_str(&line).expect("could not deserialize line");
                println!("got message from client: {:?}", payload);
            }
        }
    }

    fs::remove_file(&pipe_path).expect("could not remove pipe during shutdown");
}

fn make_loop_flag() -> Arc<AtomicBool> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("keyboard interrupted: stopping read loop");
        r.store(false, Ordering::SeqCst);
    })
    .expect("could not set up keyboard interrupt handler");

    return running;
}

/// Tries to open the pipe at `pipe_path`.
///   1. Attempt to open the path for writing
///     a. If `open_write()` fails with `io::ErrorKind::NotFound`, create the pipe and try again
///     b. If `open_write()` fails with any other error, raise the error.
///   2. Now that the file is opened for writing, ensure that it is a named pipe
///     a. If `is_fifo()` fails, panic.
///     b. If `is_fifo()` returns `false`, panic.
///   3. Return the newly opened pipe file wrapped in an `io::Result`
fn try_open<P: AsRef<Path> + Clone>(pipe_path: P) -> io::Result<fs::File> {
    let pipe = unix_named_pipe::open_read(&pipe_path);
    if let Err(err) = pipe {
        match err.kind() {
            io::ErrorKind::NotFound => {
                println!("creating pipe at: {:?}", pipe_path.clone().as_ref());
                unix_named_pipe::create(&pipe_path, Some(0o660))?;

                // Note that this has the possibility to recurse forever if creation `open_write`
                // fails repeatedly with `io::ErrorKind::NotFound`, which is certainly not nice behaviour.
                return try_open(pipe_path);
            }
            _ => {
                return Err(err);
            }
        }
    }

    let pipe_file = pipe.unwrap();
    let is_fifo = pipe_file
        .is_fifo()
        .expect("could not read type of file at pipe path");
    if !is_fifo {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "expected file at {:?} to be fifo, is actually {:?}",
                &pipe_path.clone().as_ref(),
                pipe_file.metadata()?.file_type(),
            ),
        ));
    }

    Ok(pipe_file)
}
