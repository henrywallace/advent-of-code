.PHONY: rs rs-test
rs: rs-test
	cargo build --manifest-path rs/Cargo.toml
	./rs/target/debug/dec03 input.txt --part 1
	./rs/target/debug/dec03 input.txt --part 2
rs-test:
	cargo test --manifest-path rs/Cargo.toml
