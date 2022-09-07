# rust-gd-auth
This repository is a POC auth contrib library written in rust and usable in both Python and Node. It uses the [Cobhan libaries](https://github.com/godaddy/cobhan-rust) to marshall data across FFI interfaces.

The toy contrib library implementation is in [`src/lib.rs`](./src/lib.rs). It exposes a single function, `parse` that takes in hostname configuration and a token to parse. On success it will return 0 and write a subset of the token claims into the output buffer; on failure it will return a negative integer.

[`python-test`](./python-test/) and [`node-test`](./node-test/) implement wrappers around `parse` along with a simple console program that parses a dev JWT.

Run [`./build.sh`](./build.sh) to compile the Rust library and run a simple test in both Python and Node.

