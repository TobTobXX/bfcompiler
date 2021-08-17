; ----------------------------------------------------------------------------------------
; Writes "Hello, World" to the console using only system calls. Runs on 64-bit Linux only.
; To assemble and run:
;
;     nasm -felf64 hello.asm && ld hello.o && ./a.out
; ----------------------------------------------------------------------------------------

	global		_start

	section		.text
_start:
	mov			rdi, 0x41			; pass argument
	call		_otherfunc			; call the other function

	leave							; release the stack frame
	ret								; return to the previous instruction address


_otherfunc:
	enter		0x1,0				; prepares a stack frame with 0x1 byte
	; printing stuff starts here --------------
	mov			rax, 1                  ; system call for write
	mov			rdi, 1                  ; file handle 1 is stdout
	mov			rsi, message            ; address of string to output
	mov			rdx, 13                 ; number of bytes
	syscall                             ; invoke operating system to do the write
	mov			rax, 60                 ; system call for exit
	xor			rdi, rdi                ; exit code 0
	syscall                             ; invoke operating system to exit
	; printing stuff ends here ----------------
	leave							; release the stack frame
	ret								; return to the previous instruction address



	section	.data
message:
	db		"Hello, World", 10          ; note the newline at the end
