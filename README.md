# bincode-io
Simple way to save and load Rust structs to binary files.

## Installation
```toml
[dependencies]
bincode-io = "0.1"
```

## Usage
```rust
use bincode_io::{save, load};

fn main() {
    let name = "abc";
    save("abc.bin", name).unwrap();

    let _name: String = load("abc.bin").unwrap();
    println!("{}", _name);
}
```

This crate makes it as simple as possible to serialize and deserialize data into a binary file.

## Structs
If you are serializing structs, make sure your types implement Serialize and Deserialize.
```toml
[dependencies]
bincode-io = "0.1"
serde = { version = "1.0.228", features = ["derive"] }
```

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct State { ... }
```
