# middleware-atrust-wrapper

## development

install rust using [rustup](https://rustup.rs/).

use the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension for visual studio code for ide features.

## build

run `cargo build` to build the dll. add the `--release` flag for relase builds.

dll is in `./target/(debug|release)/middleware_wrapper_atrust.dll`.

## tests

run `cargo test --all-features`. add `-- --nocapture` to the command to see stdout and stderr.

<!--
## docs

run `cargo doc --no-deps` to build the docs. add `--open` to open them in the default browser.
-->
