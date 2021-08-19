# middleware-atrust-wrapper

## development

install rust using [rustup](https://rustup.rs/).

use the `rust-analyzer` extension for visual studio code for ide features.

## build

run 

```sh
cargo build
```

add the `--release` flag for relase builds.

dll is in `./target/(debug|release)/middleware_wrapper_atrust.dll`.

## tests

run

```sh
cargo test --all-features
```

add ` -- --nocapture` to the command to see stdout and stderr.
