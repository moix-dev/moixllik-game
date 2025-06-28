set shell := ["bash", "-c"]

run:
    cargo run

build:
    cargo build --release --target wasm32-unknown-unknown
    wasm-opt -Oz -o pkg/app.wasm target/wasm32-unknown-unknown/release/moixllik-game.wasm
    llvm-strip pkg/app.wasm
    brotli -Z --suffix=.br -f pkg/app.wasm

serv:
    basic-http-server .
