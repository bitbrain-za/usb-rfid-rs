# [WIP]

# Overview

Simple function to parse raw hex data from something like /dev/hidraw0 and return a card number.
Works from products such as [this](https://www.amazon.com/Reader-125KHz-Proximity-Keyboard-Android/dp/B07TMNZPXK)

# Example

```toml
[dependencies]
usb-rfid-decoder = "1.0"
```

```rust
use usb_rfid_decoder as decoder;

fn main() {
    const CARD: [u16; 11] = [
        0x27, 0x27, 0x27, 0x1f, 0x22, 0x20, 0x27, 0x24, 0x25, 0x22, 0x28,
    ];
    let result = decoder::decode(&CARD);
    println!("{:?}", result.unwrap());
}
```

# References

- https://gist.github.com/jayliew/d3da282cb8fef44ddcb0d34227c9891e