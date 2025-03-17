launch: build-source-code
	./target/release/shell
build-source-code: clean-terminal
	cargo build --release
clean-terminal:
	clear
