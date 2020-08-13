#!/usr/bin/env just --justfile

web-build:
	cargo build --target wasm32-unknown-unknown --release

web: web-build
	mv target/wasm32-unknown-unknown/release/rust-wasm-sokoban.wasm .
	python -m http.server

desktop:
	cargo run 

clean:
	rm -rf rust-wasm-sokoban.wasm

# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
