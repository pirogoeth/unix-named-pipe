//! This is a sample named pipe client.
//! The client expects a named pipe to be created and provided as a command line
//! argument.
//! The client opens the named pipe for writing and emits randomly generated numbers
//! into the pipe, separated by newlines.

#[macro_use]
extern crate miniserde;
extern crate rand;
extern crate unix_named_pipe;

use miniserde::json;
use rand::prelude::*;
use std::io::Write;
use std::{env, thread, time};

#[derive(Debug, MiniSerialize)]
struct Message {
    numbers: Vec<u8>,
}

fn main() {
    let pipe_path = env::args()
        .nth(1)
        .expect("named pipe path required but not provided");
    println!("client opening pipe: {}", pipe_path);

    let mut pipe = unix_named_pipe::open_write(pipe_path).expect("could not open pipe for writing");

    loop {
        let payload = make_payload();
        println!("sending payload: {:?}", payload);

        let payload = json::to_string(&payload) + "\n";
        let payload = payload.as_bytes();

        let res = pipe
            .write(&payload)
            .expect("could not write payload to pipe");
        if res != payload.len() {
            println!("could not write {} bytes to pipe", payload.len());
            break;
        }

        // Not necessary, but sleep a short period of time before writing more numbers
        // to the pipe
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn make_payload() -> Message {
    let count = random::<u8>();
    let numbers: Vec<u8> = (0..count).map(|_| random::<u8>()).collect();

    Message { numbers }
}