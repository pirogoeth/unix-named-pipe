//! Provides an extension to `std::fs::File` which implements useful
//! utilities for working with FIFOs.

use std::fs;
use std::io;
use std::os::unix::fs::FileTypeExt;

/// Definitions for `std::fs::File` extensions for FIFOs
pub trait FileFIFOExt {
    fn is_fifo(&self) -> io::Result<bool>;
}

impl FileFIFOExt for fs::File {
    /// Returns a wrapped boolean to designate if the underlying
    /// file is a FIFO device.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # extern crate unix_named_pipe;
    /// # use std::fs;
    /// use unix_named_pipe::*;
    /// 
    /// # let file_name = "/tmp/fifo.5";
    /// # create(file_name, None).expect("could not create fifo");
    /// let file = open_read(file_name).expect("could not open fifo for reading");
    /// assert_eq!(file.is_fifo().unwrap(), true);
    /// # fs::remove_file(file_name).expect("could not remove fifo");
    /// ```
    fn is_fifo(&self) -> io::Result<bool> {
        let metadata = self.metadata()?;
        Ok(metadata.file_type().is_fifo())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{create, open_read};
    use super::*;
    use std::fs;

    #[test]
    fn is_fifo() {
        let file_name = "/tmp/a-fifo";
        create(file_name, None).expect("could not create fifo");

        let file = open_read(file_name).expect("could not open fifo for reading");
        assert_eq!(file.is_fifo().unwrap(), true);

        fs::remove_file(file_name).expect("could not remove fifo");
    }

    #[test]
    fn is_not_fifo() {
        let file_name = "/tmp/file.txt";
        fs::write(file_name, b"\n").expect("could not write data to file");

        let file = open_read(file_name).expect("could not open file for reading");
        assert_eq!(file.is_fifo().unwrap(), false);

        fs::remove_file(file_name).expect("could not remove file");
    }

}
