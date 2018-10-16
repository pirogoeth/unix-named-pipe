# unix-named-pipe

[![pipeline status](https://glow.dev.maio.me/sjohnson/unix-named-pipe/badges/master/pipeline.svg)](https://glow.dev.maio.me/sjohnson/unix-named-pipe/commits/master)
[![coverage report](https://glow.dev.maio.me/sjohnson/unix-named-pipe/badges/master/coverage.svg)](https://glow.dev.maio.me/sjohnson/unix-named-pipe/commits/master)

---

`unix-named-pipe` is a library to ease the creation and usage of named pipes on the Unix platform

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

## Examples

Some examples are provided in the `examples` directory. There are examples for both fixed-size messages and
for variable-sized messages
To start the example client and server, launch the server first to begin reading and then launch the client:

```shell
cargo run --example fixsz_server -- /tmp/pipe
cargo run --example fixsz_client -- /tmp/pipe
```

## Contributing

Pull requests are welcomed and encouraged.  Feel free to ask questions via the issue tracker or email.

Any contributions will be greatly appreciated <3.

## License

Licensed under MIT. See [LICENSE](/LICENSE) for details.