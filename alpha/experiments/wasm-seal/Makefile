# RUN-19 wasm-seal — orchestration targets.
# `make p2-interop` runs the cross-build goldens: builds the wasm module for
# a Node host, the native peer, then drives both from node/interop.mjs.

.PHONY: p2-interop p3-resume wasm-pkg native-peer

p3-resume: wasm-pkg native-peer
	node node/resume.mjs

wasm-pkg:
	cd crates/seal-wasm && wasm-pack build --target nodejs --dev --no-opt

native-peer:
	cargo build --release -p seal-native

p2-interop: wasm-pkg native-peer
	node node/interop.mjs

# The claim is UNSEAL-blindness: no MLS/seal crate in the server's normal
# (shipped) graph. TLS-layer crypto (quinn/rustls inside wtransport) is
# transport encryption and is expected — it cannot open MLS application
# ciphertext.
p4-blindness:
	@echo "== blind-ds NORMAL dependency graph (must contain no unseal capability) =="
	cargo tree -p blind-ds -e normal | tee /tmp/blind-ds-tree.txt
	@! grep -Ei "openmls|group-seal|group-core|seal-wire|seal-persist|seal-wasm|seal-native|lineage" /tmp/blind-ds-tree.txt \
		&& echo "BLINDNESS: no MLS/seal crate in the normal graph"
