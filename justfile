#!/usr/bin/env just --justfile
resource:
	tar -cvf resources.tar resources

web-build: 
	cargo build --target wasm32-unknown-unknown --release

web: web-build
	rm -rf rust-wasm-sokoban.wasm
	cp target/wasm32-unknown-unknown/release/rust-wasm-sokoban.wasm .
	python -m http.server

desktop:
	RUST_LOG=info cargo run 

clean:
	cargo clean
	rm -rf rust-wasm-sokoban.wasm

# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
