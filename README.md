# A-Trust interface wrapper for the fiskaltrust.Middleware
By using this native library, users who have already had implemented the A-Trust Cloud TSE can continue to use their existing implementation of A-Trust's interface to connect the fiskaltrust.Middleware's SCU packages. In Germany, our Signature Creation Units (_SCUs_) provide well-proven implementations of all currently certified TSEs.

While those SCUs are normally used by our Middleware, it's also easily possible to operate them as "stand-alone" services. If you're interested in switching to a different Cloud TSE supported by the fiskaltrust.Middleware and/or this wrapper, please reach out to our [sales team](mailto:sales@fiskaltrust.de) or visit one of our frequent [webinars](https://fiskaltrust.de/webinare).

## Usage
1. Download the latest binaries for your target OS's architecture (x64/x86) from the [releases page](releases).
2. Replace the original DLL of A-Trust with this DLL.
3. Alter the included `asigntseonline.conf` and set the `scu_url` parameter to the REST endpoint that was configured for the SCU.
    ```ini
    [default]
    scu_url = http://localhost:5000/<url-as-configured-in-portal>
    ```
4. Continue to use your existing implementation as-is.

Please refer to our [documentation platform](https://docs.fiskaltrust.cloud) about how to onboard PosOperators and creating and rolling out Middleware instances.

## Development
First, install Rust using [rustup](https://rustup.rs/). For development purposes, the normal `stable` toolchain is sufficient - for building the x86 DLL, the `stable-i686` toolchain is needed (which can be installed via `rustup install stable-i686`).

We also recommend using the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension in case you are using VS Code to get optimal IDE support.

### Build
Run `cargo +stable-i686 build` to build the dll. Add the `--release` flag for release builds.

The built DLL is stored in `./target/(debug|release)/middleware_wrapper_atrust.dll`.

### Tests
Run `cargo test --all-features`. Add `-- --nocapture` to the command to see stdout and stderr.

To run the tests on a real fiskaltrust.SCU, set the `SCU_URL` environment variable to the SCU's REST endpoint.
