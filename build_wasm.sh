cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./wasm/ --out-name "physics_engine" \
  ./target/wasm32-unknown-unknown/release/physics_engine.wasm
