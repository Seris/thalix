.PHONY: build rebuild clean clean-core run debug dump \
	clean-cargo clean-nasm build-cargo full-rebuild kill-qemu

KERNEL=build/isoroot/boot/thalix

NASM_SRC=$(shell find src/ -name "*.nasm")
NASM_OBJ=$(patsubst %.nasm, build/%.o.asm, $(NASM_SRC))
RUST_SRC=$(shell find src/ -name "*.rs")
RUST_KERNEL=target/x86_64-thalix/release/libthalix.a
XARGO=xargo
CARGO=cargo

build: build-cargo $(KERNEL)
rebuild: clean-cargo clean-nasm build
full-rebuild: clean-core rebuild

run: build thalix.iso
	qemu-system-x86_64 -cdrom thalix.iso

debug: build thalix.iso kill-qemu
	qemu-system-x86_64 -cdrom thalix.iso -s -S -pidfile qemu.pid &
	- rust-gdb $(KERNEL) -ex "target remote localhost:1234"
	make kill-qemu

kill-qemu:
	@ if [ -a qemu.pid ]; \
	then \
		echo "killing qemu"; \
		kill `cat qemu.pid`; \
		rm qemu.pid; \
	fi


build-cargo:
	@ cp build/x86_64-thalix.json .
	$(XARGO) build --target=x86_64-thalix --release
	@ rm x86_64-thalix.json

thalix.iso: $(KERNEL)
	grub-mkrescue -o thalix.iso build/isoroot \
		>grub-mkrescue.log 2>&1 grub-mkrescue.log

clean-cargo:
	$(CARGO) clean

clean-nasm:
	rm -rf build/src

clean-core:
	$(XARGO) clean

dump:
	objdump -D $(KERNEL)

$(KERNEL): $(NASM_OBJ) $(RUST_KERNEL)
	ld -n -T build/linker.ld -o $@ $^ --gc-sections -nostdlib

build/%.o.asm: %.nasm
	@ mkdir -p $(shell dirname $@)
	nasm -g -f elf64 $< -o $@
