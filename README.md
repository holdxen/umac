# flatline-umac

Rust bindings for UMAC (Universal Message Authentication Code).

## Overview

UMAC is a fast and provably secure message authentication code. This crate provides Rust bindings to the UMAC C library, offering both 64-bit and 128-bit tag variants.

The core C implementation is sourced from [OpenSSH](https://github.com/openssh/openssh-portable).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
flatline-umac = "0.1.0"
```

## Usage

```rust
use flatline_umac::{UMac, UMac64, UMac128};

fn main() {
    // Using UMAC-64 (8-byte tag)
    let key = [0u8; 16];
    let mut mac = UMac64::new(key);
    mac.update(b"Hello, World!");
    let nonce = [0u8; 8];
    let tag = mac.finalize(nonce);
    println!("Tag: {:?}", tag);

    // Using UMAC-128 (16-byte tag)
    let mut mac = UMac128::new(key);
    mac.update(b"Hello, World!");
    let tag = mac.finalize(nonce);
    println!("Tag: {:?}", tag);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.