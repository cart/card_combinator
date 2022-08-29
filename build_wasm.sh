cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web target/wasm32-unknown-unknown/debug/card_combinator.wasm --out-dir ./wasm