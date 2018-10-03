//! Provides utilities for working with Unix named pipes / FIFOs.
extern crate libc;

use errno;
use libc::{c_int, mkfifo, mode_t, EACCES, EEXIST, ENOENT};
use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

/// Creates a new named pipe at the path given as `path`.
/// Pipe will be created with mode `mode` if given, else `0o644` will be used.
pub fn create<P: AsRef<Path>>(path: P, mode: Option<u32>) -> io::Result<()> {
    let path = CString::new(path.as_ref().to_str().unwrap())?;
    let mode = mode.unwrap_or(0o644);
    let result: c_int = unsafe { mkfifo(path.as_ptr(), mode as mode_t) };

    let result: i32 = result.into();
    if result == 0 {
        return Ok(());
    }

    let error = errno::errno();
    match error.0 {
        EACCES => {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("could not open {:?}: {}", path, error),
            ));
        }
        EEXIST => {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("could not open {:?}: {}", path, error),
            ));
        }
        ENOENT => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("could not open {:?}: {}", path, error),
            ));
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("could not open {:?}: {}", path, error),
            ));
        }
    }
}

/// Opens a named pipe for reading
pub fn open_read<P: AsRef<Path>>(path: P) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(path)
}

/// Opens a named pipe for writing
pub fn open_write<P: AsRef<Path>>(path: P) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .append(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(path)
}

#[cfg(test)]
mod tests {
    extern crate fs2;

    use fs2::FileExt;
    use std::fs;
    use std::io::{self, Error, ErrorKind, Read, Write};
    use super::*;

    fn lock_active_test() -> io::Result<fs::File> {
        let file = File::create("/tmp/unix-named-pipe_tests.lock")?;
        file.lock_exclusive()?;

        Ok(file)
    }

    #[test]
    fn create_new_pipe() {
        let lock = lock_active_test().unwrap();

        let filename = "/tmp/pipe";
        let _ = create(filename, None).expect("could not create pipe");

        fs::remove_file(filename).expect("could not remove test pipe");
        lock.unlock().unwrap();
    }

    #[test]
    fn create_pipe_eexists() {
        let lock = lock_active_test().unwrap();

        let filename = "/tmp/pipe";
        fs::write(filename, "").expect("could not write test file");

        let pipe = create(filename, None);
        assert_eq!(pipe.is_err(), true);

        let err: Error = pipe.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::AlreadyExists);

        fs::remove_file(filename).expect("could not remove test file");
        lock.unlock().unwrap();
    }

    #[test]
    fn create_pipe_enoent() {
        let filename = "/notadir/pipe";
        let pipe = create(filename, None);
        assert_eq!(pipe.is_err(), true);

        let err: Error = pipe.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn open_pipe_read() {
        let lock = lock_active_test().unwrap();

        let filename = "/tmp/test.pipe";
        let _ = create(filename, None).expect("could not make test pipe");

        let contents: [u8; 4] = [0xca, 0xfe, 0xba, 0xbe];
        let mut actual: [u8; 4] = [0; 4];

        // Create a reader first
        let mut read_file = open_read(filename).expect("could not open test pipe for reading");

        // Write some data to the pipe
        {
            let mut write_file = open_write(filename).expect("could not open test pipe for writing");
            write_file.write(&contents).expect("could not write test data to pipe");
            write_file.flush().expect("could not flush test pipe");
        }

        // Read some data from the pipe
        read_file.read_exact(&mut actual).expect("could not read test data from pipe");
        assert_eq!(contents, actual);

        fs::remove_file(filename).expect("could not remove test file");
        lock.unlock().unwrap();
    }
}
