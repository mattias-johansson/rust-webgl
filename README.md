
Compile and package:
$ cargo build --target wasm32-unknown-unknown --release &&   wasm-bindgen ./target/wasm32-unknown-unknown/release/webgl.wasm --out-dir . --no-typescript --no-modules


Start server and disable caching:
$ http-server -c-1

