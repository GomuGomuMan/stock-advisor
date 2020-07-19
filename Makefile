default: run

build:
	@cargo check
	@cargo build

run:
	@cargo run

release:
	@cargo build --release