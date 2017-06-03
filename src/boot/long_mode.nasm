global setup_long_mode

extern kernel_main

section .text
bits 32

enable_paging:
	; loading Page Map level 4 address in cr3
	mov eax, paging_pml4_table
	mov cr3, eax

	; enabling physical-address extension
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	; set long mode enable bit in EFER MSR register
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

	; enabling paging !
	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax

	ret

; identity mapping up to 1GB
setup_early_paging:
	; filling up the first entry of the Page Map level 4 table
	mov eax, paging_pdirptr_table
	or eax, 0x3 ; present - R/W
	mov [paging_pml4_table], eax

	; filling up the first entry of a Page Directory Pointer table
	mov eax, paging_pdir_table
	or eax, 0x3 ; present - R/W
	mov [paging_pdirptr_table], eax

	; filling up a page directory table (which store physical address)
	mov ecx, 0
	.map_p2_table:
		; at each iteration, we setup an entry of the next 2MB
		; (starting at 0 up to 1GB)
		mov eax, 0x200000 ; 2 MB
		mul ecx
		or eax, 0b10000011 ; present - R/W - page size flag

		mov [paging_pdir_table + ecx * 8], eax

		; after 512 iterations, we return
		inc ecx
		cmp ecx, 512
		jne .map_p2_table
	ret

setup_long_mode:
	call setup_early_paging
	call enable_paging
	; we are now in 32 bit compatibility mode
setup_gdt:
	lgdt [gdt64.pointer]

	; update CS register
	jmp gdt64.code:world64

bits 64
world64:
cli
	mov ax, gdt64.data
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; let us go in the 64 bit world ! bye bye 32 bit !
	jmp kernel_main



section .ro_data
gdt64:
	.null: equ $-gdt64
	dd 0 ; limit/base low
	db 0 ; base middle
	db 0 ; access (we can't access the null descriptor anyway ^^)
	db 0 ; limit high / flags
	db 0 ; base high
	.code: equ $-gdt64
	dd 0 ; limit/base low
	db 0 ; base middle
	db (1 << 7) | (1 << 4) | (1 << 3) | (1 << 1)  ; access flags
	db (1 << 5) ;limit high / flags
	db 0 ; base high
	.data: equ $-gdt64
	dd 0 ; limit/base low
	db 0 ; base middle
	db (1 << 7) | (1 << 4) | (1 << 1)  ; access flags
	db 0 ; limit high / flags
	db 0 ; base high
	.pointer:
	dw $ - gdt64 - 1 ; gdt length - 1
	dq gdt64

section .bss
align 4096
paging_pml4_table:
	resb 4096
paging_pdirptr_table:
	resb 4096
paging_pdir_table:
	resb 4096
