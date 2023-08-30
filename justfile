mod app 'openvtt_app'
mod server 'openvtt_server'

help:
  @just --list

[no-cd]
dev:
  just app dev&
  just server dev

[no-cd]
run: wasm-build
  just server run

[no-cd]
wasm-build:
	just app wasm-build
	wasm-bindgen --out-dir openvtt_server/public/ --target web target/wasm32-unknown-unknown/debug/openvtt_app.wasm
	cp -r openvtt_app/assets openvtt_server/public/assets
