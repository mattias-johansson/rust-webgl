
Compile and package:
$ cargo build --target wasm32-unknown-unknown --release && wasm-bindgen ./target/wasm32-unknown-unknown/release/webgl.wasm --out-dir . --no-typescript --target no-modules


Start server and disable caching:
$ http-server -c-1


[lib]
crate-type = ["cdylib"]

[dependencies]
js-sys = "0.3.39"
wasm-bindgen = "0.2.62"
nalgebra = "0.18.0"
nalgebra-glm = "=0.4.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
erased-serde = "0.3"
rand = "0.7.3"
uuid = { version = "0.8", features = ["v4", "wasm-bindgen", "v1"] }

[dependencies.web-sys]
version = "0.3.4"