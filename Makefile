run:
	cargo run --bin day$(DAY) -- day$(DAY)/.inputs/$(SIZE)

run-release:
	cargo build --release --bin day$(DAY)
	./target/release/day$(DAY) day$(DAY)/.inputs/$(SIZE)

new:
	cargo new day$(DAY) --bin --vcs none
	@echo "aoc.workspace = true" >> day$(DAY)/Cargo.toml
	@echo "owo-colors.workspace = true" >> day$(DAY)/Cargo.toml
	@echo "itertools.workspace = true" >> day$(DAY)/Cargo.toml

	@echo "\n[dev-dependencies]\ndivan.workspace = true" >> day$(DAY)/Cargo.toml 

	@echo "\n[[bench]]\nname = \"benchmark\"\nharness = false" >> day$(DAY)/Cargo.toml 

	@mkdir day$(DAY)/.inputs

	@mkdir day$(DAY)/benches
	@touch day$(DAY)/benches/benchmark.rs
	@echo "fn main() {\n\tdivan::main();\n}" >> day$(DAY)/benches/benchmark.rs

test:
	cargo test --bin day$(DAY)

bench:
	cargo bench --bin day$(DAY)
