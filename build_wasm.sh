cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web target/wasm32-unknown-unknown/debug/cart-does-a-jam.wasm --out-dir ./wasm