## maliput-rs

This repos serves as an exploration of Rust bindings using maliput (C++) as guinea pig.


### Prerequisites

1. Install rustc

### Packages
 - maliput-src: Vendors maliput package
 - maliput-sys: Rust bindings
 - maliput-src: Vendors maliput_malidrive package

### Usage

1. Git clone
```
git clone --recursive https://github.com/francocipollone/maliput-rs.git
cd maliput-rs
```

2. Build everything

```
cargo build
```

3. Indicate the location of the maliput_malidrive road network plugin:

```sh
export MALIPUT_PLUGIN_PATH=$(cargo run --bin maliput_malidrive-src)
```

4. Run example
```
cargo run --example create_rn
```
Output should be:
```
$ cargo run --example create_rn
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/create_rn`

Executing create_rn.rs example: 
[INFO] Plugin Id: maliput_malidrive was correctly loaded.

[INFO] A new version of Plugin Id: maliput_malidrive was loaded.

[INFO] Number of plugins loaded: 1

linear_tolerance: 0.05
angular_tolerance: 0.01
num_junctions: 4
num_branch_points: 12
```
