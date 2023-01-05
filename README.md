# ESP-IDF application with Rust component

This is ESP-IDF project which contains Rust Bare Metal (no_std) part as a component.

How to build:

```
cd components/example_rust
cargo build --release
cd ../../
idf.py build flash monitor
```
