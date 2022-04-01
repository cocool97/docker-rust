# docker-rust

Rust crate providing HTTP bindings to Docker API. Can be used to talk directly with Docker daemon, without using shell commands.

Currently aimed Docker API version: `1.41`.

## How does it work ?

```rust
// First create a DockerInstance, used to talk directly with Docker daemon.
// Can also be configured.
let mut instance = DockerInstanceBuilder::new().build().unwrap();

// Directly call methods, returning concrete types.
instance.list_images(false, None, false);
```