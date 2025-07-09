# SSH Server

This is a personal exploration project using
[`russh`](https://github.com/warp-tech/russh), a pure-Rust async SSH server
library. The server is minimal and supports:

- Basic public key authentication (all keys accepted)
- Session channels and simple data echoing
- TCP/IP forwarding (example response only)

## Running

Ensure Rust is installed (`rustup` recommended), then:

```bash
cargo run
```

This will start the SSH server on:
```0.0.0.0:2222```

This project uses `env_logger` for debug output. Set the log level via the
filter_level: 
```rs
env_logger::builder()
    .filter_level(log::LevelFilter::Debug)
    .init();
```
