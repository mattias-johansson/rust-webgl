cargo build --target wasm32-unknown-unknown --release 
wasm-bindgen ./target/wasm32-unknown-unknown/release/webgl.wasm --out-dir . --no-typescript --target no-modules;
cd src/public
cargo build --target wasm32-unknown-unknown --release 
cd ../..
wasm-bindgen ./src/public/target/wasm32-unknown-unknown/release/webgl_api.wasm --out-dir . --no-typescript --target no-modules
