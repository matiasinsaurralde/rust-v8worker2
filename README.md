## rust-v8worker2

This is a minimal binding between Rust (and V8 JavaScript. Basic concept is to only expose two methods to JavaScript: send and receive.

Based on [ry/v8worker2](https://github.com/ry/v8worker2).

## Usage

Add this to your `Config.toml`:
```toml
[dependencies]
v8worker2 = { git = "https://github.com/matiasinsaurralde/rust-v8worker2" }
```

```rust
extern crate v8worker2;
use v8worker2::v8worker;
pub fn cb(_data: *mut u8, _length: i32, _index: i32) {
    // This will print 5, the length of the ArrayBuffer we're sending in main:
    println!("cb: {}", _length);
}

fn main() {
    unsafe {
        v8worker2::v8_init();
        v8worker2::CB = cb;
    }
    let mut test_worker = v8worker::new();
    let code = String::from("V8Worker2.send(new ArrayBuffer(5))");
    let script_name = String::from("code2.js");
    test_worker.load(script_name, code);
}
```

## Requirements

- Set `V8_INCLUDE` (`v8/include`) and `V8_BUILD` (contains `lib_v8monolith.a`)