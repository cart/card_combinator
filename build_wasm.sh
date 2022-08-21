cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web target/wasm32-unknown-unknown/debug/bevy_jam_2.wasm --out-dir ./wasm