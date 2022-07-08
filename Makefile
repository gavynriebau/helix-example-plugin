

all: wasm
	cp ./target/wasm32-wasi/debug/helix_example_plugin.wasm ~/.config/helix/plugins/

wasm:
	cargo build
	

