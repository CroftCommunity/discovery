# RUN-19 wasm-seal — orchestration targets.
# `make p2-interop` runs the cross-build goldens: builds the wasm module for
# a Node host, the native peer, then drives both from node/interop.mjs.

.PHONY: p2-interop wasm-pkg native-peer

wasm-pkg:
	cd crates/seal-wasm && wasm-pack build --target nodejs --dev --no-opt

native-peer:
	cargo build --release -p seal-native

p2-interop: wasm-pkg native-peer
	node node/interop.mjs
