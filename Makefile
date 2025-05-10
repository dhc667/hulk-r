SRC_FILES := $(shell find . -type f -not -path './target/release/runner')

./target/release/runner: $(SRC_FILES)
	cargo build --release --quiet

.PHONY: compile
compile: ./target/release/runner
	./target/release/runner

.PHONY: execute
execute: compile
	lli main.ll

