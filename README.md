## maliput-rust

This repos serves as an exploration of Rust bindings using maliput (C++) as guinea pig.


### Prerequisites

1. Install rustc

### Packages
 - maliput-src: Vendors maliput package
 - maliput-sys: Rust bindings

### Usage

1. Git clone
```
git clone --recursive https://github.com/francocipollone/maliput-rust.git
cd maliput-rust
```

2. Build everything

```
cargo build
```

3. Run example
```
cargo --example run saturate
```
