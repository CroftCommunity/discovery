# tier-proof — RUN-17 make targets.
#
# `make roles-up` co-hosts the three delivery-role PROCESSES (P8, §A.7) the way
# one VPS would run them: the web-native DS, the swarm-peer, and the
# history-convergence node, each its own process under a distinct port. Killing
# any one leaves the other two serving (failure isolation) — the property the
# `p8_processes.rs` integration test asserts executably.

.PHONY: check test roles-up roles-down

check: test

test:
	cargo test
	cargo clippy --all-targets -- -D warnings
	cargo fmt --check

# Launch the three roles as separate processes. Each prints `PORT <n>` then
# serves; PIDs are recorded so `roles-down` can stop them.
roles-up:
	@cargo build --bins
	@echo "starting delivery roles (DS, swarm-peer, convergence) as separate processes…"
	@./target/debug/ds        & echo $$! > .ds.pid
	@./target/debug/swarm_peer & echo $$! > .swarm.pid
	@./target/debug/convergence & echo $$! > .conv.pid
	@echo "three processes up; kill any one and the other two keep serving."
	@echo "PIDs: DS=$$(cat .ds.pid) swarm=$$(cat .swarm.pid) convergence=$$(cat .conv.pid)"

roles-down:
	@for p in .ds.pid .swarm.pid .conv.pid; do \
		[ -f $$p ] && kill $$(cat $$p) 2>/dev/null; rm -f $$p; \
	done
	@echo "roles stopped."
