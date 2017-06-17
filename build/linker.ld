ENTRY(kernel_entry)

SECTIONS {
	. = 1M;

	.multiboot_header : {
		KEEP(*(.multiboot_header))
	}

	.rodata : {
		*(.rodata .rodata.*)
	}

	.data.rel.ro : {
		*(.data.rel.ro .data.rel.ro.*)
	}

	.text : {
		*(.text .text.*)
	}
}
