ENTRY(kernel_entry)

SECTIONS {
	. = 1M;

	.multiboot_header : {
		KEEP(*(.multiboot_header))
	}

	.text : {
		*(.text)
	}
}
