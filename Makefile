.PHONY: build
build:
	cd boot && cargo build
	cd kernel && cargo build

.PHONY: clean
clean:
	cd boot && cargo clean
	cd kernel && cargo clean

.PHONY: gdb
gdb:
	gdb -ex "target remote localhost:1234"
