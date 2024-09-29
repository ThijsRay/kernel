ARCH := x86_64

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: clean
clean:
	cargo clean

.PHONY: gdb
gdb:
	gdb -ex "target remote localhost:1234"
