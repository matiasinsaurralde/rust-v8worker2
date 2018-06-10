## rust-v8worker2

This is a minimal binding between Rust (and V8 JavaScript. Basic concept is to only expose two methods to JavaScript: send and receive.

Based on [ry/v8worker2](https://github.com/ry/v8worker2).

## Usage

Add this to your `Config.toml`:
```
[dependencies]
v8worker2 = { git = "https://github.com/matiasinsaurralde/rust-v8worker2" }
```

## Requirements

- Set `V8_INCLUDE` (`v8/include`) and `V8_BUILD` (contains `lib_v8monolith.a`)