ARCH := x86_64

.PHONY: build
build:
	cargo bootimage --target arch/$(ARCH).json

.PHONY: run
run: build
	qemu-system-x86_64 -drive format=raw,file=target/$(ARCH)/debug/bootimage-kernel.bin

.PHONY: debug
debug: build
	qemu-system-x86_64 -s -S -drive format=raw,file=target/$(ARCH)/debug/bootimage-kernel.bin

.PHONY: clean
clean:
	cargo clean

.PHONY: gdb
gdb: build
	gdb -ex "target remote localhost:1234" target/$(ARCH)/debug/kernel
