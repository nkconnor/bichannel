# bichannel

A zero dependency `std::sync::mpsc` based bidirectional channel. Each side can send
and receive with its counterpart

### Getting Started

```toml
bichannel = "1"
```

**Example Usage**

```rust
let (left, right) = bichannel::channel();

// Send from the left to the right
left.send(1).unwrap();
assert_eq!(Ok(1), right.recv());

// Send from the right to the left
right.send(2).unwrap();
assert_eq!(Ok(2), left.recv());
```

### License
TODO MIT/APACHE

### Contributing

Bug reports, feature requests, and contributions are warmly welcomed.

NOTE: This README uses [cargo-readme](https://github.com/livioribeiro/cargo-readme). To
update the README, use `cargo readme > README.md`
