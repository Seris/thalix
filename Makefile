.PHONY: build rebuild clean clean-core run reun-debug dump \
	clean-cargo clean-nasm build-cargo full-rebuild

KERNEL=build/isoroot/boot/thalix

NASM_SRC=$(shell find src/ -name "*.nasm")
NASM_OBJ=$(patsubst %.nasm, build/%.o.asm, $(NASM_SRC))
RUST_SRC=$(shell find src/ -name "*.rs")
RUST_KERNEL=target/x86_64-thalix/debug/libthalix.a
XARGO=xargo
CARGO=cargo

build: build-cargo $(KERNEL)
rebuild: clean-cargo clean-nasm build
full-rebuild: clean-core rebuild

run: build thalix.iso
	qemu-system-x86_64 -cdrom thalix.iso -d int,cpu_reset

build-cargo:
	@ cp build/x86_64-thalix.json .
	- $(XARGO) build --target=x86_64-thalix
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
	ld -n -T build/linker.ld -o $@ $^ --gc-sections

build/%.o.asm: %.nasm
	@ mkdir -p $(shell dirname $@)
	nasm -g -f elf64 $< -o $@
