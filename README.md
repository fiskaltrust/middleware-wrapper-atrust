# A-Trust interface wrapper for the fiskaltrust.Middleware
By using this native library, users who have already had implemented the A-Trust Cloud TSE can continue to use their existing implementation of A-Trust's interface to connect the fiskaltrust.Middleware's SCU packages. In Germany, our Signature Creation Units (_SCUs_) provide well-proven implementations of all currently certified TSEs.

While those SCUs are normally used by our Middleware, it's also easily possible to operate them as "stand-alone" services. If you're interested in switching to a different Cloud TSE supported by the fiskaltrust.Middleware and/or this wrapper, please reach out to our [sales team](mailto:sales@fiskaltrust.de) or visit one of our frequent [webinars](https://fiskaltrust.de/webinare).

3. Alter the included `asigntseonline.conf` and set the `scu_url` parameter to the REST endpoint that was configured for the SCU.
## Usage
1. Download the latest binaries for your target OS's architecture (x64/x86) from the [releases page](https://github.com/fiskaltrust/middleware-wrapper-atrust/releases) or [build the binaries from source](#building-from-source).
2. Replace the original DLL of A-Trust with this DLL.
3. Alter the included `asigntseonline.conf` and set the `scu_url` parameter to the REST endpoint that was configured for the SCU.
    ```ini
    [default]
    scu_url = http://localhost:5000/<url-as-configured-in-portal>
    ```
4. Continue to use your existing implementation as-is.

Please refer to our [documentation platform](https://docs.fiskaltrust.cloud) about how to onboard PosOperators and creating and rolling out Middleware instances.

### Building from source

#### Prerequisites
* A [rust](https://www.rust-lang.org/) compiler (tested with version >= 1.54)

#### Build dll

Install rust using [rustup](https://rustup.rs/). For development the normal `stable` toolchain is enough. For building a x86 dll the `stable-i686` toolchain is needed. (install with `rustup install stable-i686`).

Run `cargo build --release` for x64 dll or `cargo +stable-i686 build --release` for x86 systems.

The built dll can be found in here `./target/release/middleware_wrapper_atrust.dll`.

> ***Note:** To get a smaller dll the library can be compiled with the `--no-default-features` flag. With this flag all of the unimplemented functions will not be included in the final dll.*

<!--
## Docs

Run `cargo doc --no-deps` to build the docs. add `--open` to open them in the default browser.
-->

## Contributing

Use the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension for visual studio code for IDE features.

### Tests

Run `cargo test --all-features`. Add `-- --nocapture` to the command to see stdout and stderr.

To run the tests on a real fiskaltrust.SCU set the `SCU_URL` environment variable to the SCU REST endpoint (e.g. `http://localhost:1401/scu_url`).

## License

This project is licensed under the MIT License - see the [`LICENSE`](./LICENSE) file for details
