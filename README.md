# unix-named-pipe

A library to ease creation of named pipes on the Unix platform

## Usage

```rust
extern crate unix_named_pipe;

...

let filename = "/var/run/application.pipe";
let mode: u32 = 0o644

// Create a new named pipe
unix_named_pipe::create(filename, mode)?;

// Open a named pipe for reading
let read_file = unix_named_pipe::open_read(filename)?;

// Open a named pipe for writing (appending)
let write_file = unix_named_pipe::open_write(filename)?;
```

## Contributing

Pull requests are welcomed and encouraged.  Feel free to ask questions via the issue tracker or email.

Any contributions will be greatly appreciated <3.

## License

Licensed under MIT. See [LICENSE](/LICENSE) for details.