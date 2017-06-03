section .multiboot_header
header_start:
	dd 0xe85250d6 ; multiboot2 magic
	dd 0 ; architecture (x86_64)
	dd header_end - header_start
	dd 0x100000000 - (0xe85250d6 + 0 + header_end - header_start)

	dw 0
	dw 0
	dd 8
header_end:

global kernel_entry
extern setup_long_mode

section .text
bits 32

kernel_entry:
	xor ebp, ebp
	mov esp, stack_start
	cli

	call check_multiboot
	call check_cpuid
	call check_longmode

	; long mode is available, multiboot2 header is present
	; let us go on an adventure ! :)
	jmp setup_long_mode

	; we are now in long mode ! c:
	mov dword [0xb8000], 0x2f4b2f4f
	hlt

check_multiboot:
	cmp eax, 0x36d76289 ; multiboot2 assure that eax must contain this magic
	mov al, 'M'
	jne early_error
	ret

check_cpuid:
	pushfd
	pop eax
	mov ebx, eax ; copy flag in ebx for future comparaison
	xor eax, 1 << 21 ; flip 21th bit
	push eax
	popfd ; set flags register

	pushfd
	pop eax
	cmp eax, ebx
	mov al, 'C'
	je early_error ; CPUID not available because 21th bit of flags was not flipped
	ret

check_longmode:
	mov eax, 0x80000000
	cpuid
	cmp eax, 0x80000001
	mov al, 'L'
	jb early_error ; if longmode available, we must have function above this value
	ret

; print a ascii as error (which is stored in al) and halt
early_error:
	mov dword [0xB8000], 0x04720445
	mov dword [0xB8004], 0x043A0472
	mov dword [0xB8008], 0x04200420
	mov byte [0xB800A], al
	mov word [0xB800C], 0x0420
	hlt


section .bss
; stack is growing from high to low address
stack_end:
	resb 4096
stack_start:
