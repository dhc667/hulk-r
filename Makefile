./target/release/runner: Cargo.toml ./runner/src/main.rs ./runner/src/lib.rs ./runner/src/runner.rs
	cargo build --release

.PHONY: compile
compile: ./target/release/runner
	./target/release/runner

.PHONY: execute
execute: compile
	lli main.ll

