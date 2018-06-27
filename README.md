## rust-v8worker2

[![Build Status](https://travis-ci.org/matiasinsaurralde/rust-v8worker2.svg?branch=master)](https://travis-ci.org/matiasinsaurralde/rust-v8worker2)

This is a minimal binding between Rust (and V8 JavaScript. Basic concept is to only expose two methods to JavaScript: send and receive.

Based on [ry/v8worker2](https://github.com/ry/v8worker2).

## Usage

Add this to your `Config.toml`:
```toml
[dependencies]
v8worker2 = { git = "https://github.com/matiasinsaurralde/rust-v8worker2" }
```

**Note:** If you include this repository as a dependency, the `cargo build` command may take some time to fetch and build V8, on my box it takes around 20 minutes. There's a condition that prevents this build step to run if the library is already present. If detailed build logging is required consider using `cargo build -vv`.

To use the crate:

```rust
extern crate v8worker2;
extern crate bytes;

use v8worker2::worker::Worker as Worker;

fn main() {
    // Initialize V8 (V8::InitializePlatform and V8::Initialize):
    let mut _handler = v8worker2::new_handler();
    _handler.init();

    // Setup a callback that receives bytes and returns boxed bytes:
    let cb = |incoming_data: bytes::Bytes| -> Box<bytes::Bytes> {
        println!("Getting data from V8, length is {}", incoming_data.len());

        // Send some stuff to V8, this is not in use at the moment but we still require it:
        let data = Bytes::from(&b"reply"[..]);
        Box::new(data)
    };

    // Initialize a worker with the callback:
    let mut worker = Worker::new(cb);

    // Send an empty ArrayBuffer (V8 -> Rust), the callback will print the length of it:
    worker.load("code.js", "V8Worker2.send(new ArrayBuffer(10))".to_string());
}
```

## Setup

If you want to see the detailed output of the V8 build, run:
```
$ ./build.py --enable-ccache
```

Otherwise you may run `cargo build` directly, the V8 build steps are part of `build.rs`.

To run the tests, use `cargo test -- --nocapture`.

## Requirements

- Python (required by the build script)
- C/C++ compiler
- pkg-config
- ccache (OS X: `brew install ccache`)
- Git
- Rust 1.26 or newer

## Supported platforms

My development box runs OS X, the CI environment runs Linux. These two platforms should me ok.

## License

This project contains code written by [Ryan Dahl](https://github.com/ry), I'm using the same license for my additional Rust codebase: 

[MIT](LICENSE)