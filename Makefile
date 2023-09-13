release ?=

$(info release is $(release))

ifdef release
  release_flag :=--release
  target_dir :=release
else
  release_flag :=
  target_dir :=debug
endif

run-server:
	cargo run -p openvtt_server

run-app:
	cargo run -p openvtt_app

build-app:
	cargo build -p openvtt_app $(release_flag) --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./static/ --target web ./target/wasm32-unknown-unknown/$(target_dir)/openvtt_app.wasm
	cp openvtt_server/www/index.html static/index.html
	cp -r openvtt_app/assets static/assets

watch:
	systemfd --no-pid -s http::3000 -- cargo watch -x "run -p openvtt_server"

all: build-app run-server

.PHONY: all run build watch
