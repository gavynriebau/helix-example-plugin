

all: wasm
	cp ./target/wasm32-wasi/debug/helix_example_plugin.wasm ~/.config/helix/plugins/

wasm: protobuf-files
	cargo build
	
protobuf-files:
	cp -r ../helix/helix-plugins/src/generated ./src/

