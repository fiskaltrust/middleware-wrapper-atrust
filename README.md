# middleware-wrapper-atrust

## development

install rust using [rustup](https://rustup.rs/). For development the normal `stable` toolchain is enough. For building the x86 dll the `stable-i686` toolchain is needed. (install with `rustup install stable-i686`).

use the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension for visual studio code for ide features.

## build

run `cargo +stable-i686 build` to build the dll. add the `--release` flag for relase builds.

dll is in `./target/(debug|release)/middleware_wrapper_atrust.dll`.

## tests

run `cargo test --all-features`. add `-- --nocapture` to the command to see stdout and stderr.

to run the tests on a real fiskaltrust.SCU set the `SCU_URL` environment variable to the SCU rest endpoint.

<!--
## docs

run `cargo doc --no-deps` to build the docs. add `--open` to open them in the default browser.
-->
