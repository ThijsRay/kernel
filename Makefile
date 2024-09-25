ARCH := x86_64

.PHONY: build
build:
	cargo bootimage --target arch/$(ARCH).json

.PHONY: run
run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-kernel.bin

.PHONY: clean
clean:
	cargo clean
