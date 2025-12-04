run:
	cargo run --bin day$(DAY) -- day$(DAY)/.inputs/$(SIZE)
new:
	cargo new day$(DAY) --bin --vcs none
	echo "aoc.workspace = true\nowo-colors.workspace = true\nitertools.workspace = true" >> day$(DAY)/Cargo.toml
	mkdir day$(DAY)/.inputs
