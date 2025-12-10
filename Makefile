# Makefile for Rust-based POSIX-compatible Kernel

TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/rpos-kernel
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

.PHONY: all build clean disasm run

all: build

build:
	@cargo build --release

disasm: build
	@$(OBJDUMP) -S $(KERNEL_ELF) > $(DISASM_TMP)

kernel: build
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-device loader,file=$(KERNEL_ELF),addr=0x80200000

clean:
	@cargo clean
	@rm -f $(KERNEL_BIN)

help:
	@echo "Rust-based POSIX-compatible Kernel"
	@echo "Available targets:"
	@echo "  build    - Build the kernel"
	@echo "  run      - Build and run the kernel in QEMU"
	@echo "  disasm   - Generate disassembly"
	@echo "  clean    - Clean build artifacts"
