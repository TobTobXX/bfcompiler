	global		_start

	section		.text
_start:
	enter		1, 0
	; call		_input
	; mov			[rbp-1], rax
	; mov			rax, [rbp-1]
	; call		_output

	call _alloc
	mov			rdi, rax		; Output to input
	mov			rsi, 256		; len: 256
	call _free
	add			rdi, 256		; next 256
	mov			rsi, 256		; len 256
	call _free

	mov			rdi, 0			; Return 0
	jmp			_exit

_alloc:
	enter		0, 0

	mov			rax, 9			; syscall 9 = mmap
	mov			rdi, 0			; arg: addr = 0 lets kernel set start address
	mov			rsi, 512		; arg: len = 512
	mov			rdx, 1			; arg: Allow reading of page
	or			rdx, 2			;      Allow writing too
	mov			r10, 2			; arg: process-private mapping
	or			r10, 20			;      not a file-backed mapping
	mov			r8, -1			; arg: fd (unused)
	mov			r9, 0			; arg: offset (unused)
	syscall

	cmp			rax, -1			; If return value == -1
	jne			success			; exit with error
	mov			rdi, 1			; exit with 1
	jmp			_exit
success:

	leave
	ret

_free:
	enter		0, 0

	mov			rax, 11			; syscall 11
	; addr is already in rdi
	; len is already in rsi
	syscall

	leave
	ret

; ; Output the char in rax to stdout
; _output:
; 	enter		1, 0
; 	mov			[rbp-1], rax	; save rax to stack
; 	mov			rax, 1			; syscall number: 1 for write
; 	mov			rdi, 1			; arg: file descriptor: 1 is stdout
; 	mov			rsi, rbp		; arg: prointer to buffer
; 	sub			rsi, 1			;      we want rbp-1
; 	mov			rdx, 1			; arg: only write one byte
; 	syscall
; 	leave
; 	ret

; ; Blocking on one char input and storing it into rax
; ; Note: To flush the stdin, the terminal will have to receive a newline
; _input:
; 	enter		1, 0
; 	mov			rax, 0			; syscall number: 0 for read
; 	mov			rdi, 0			; arg: file descriptor: 0 is stdin
; 	mov			rsi, rbp		; arg: prointer to buffer
; 	sub			rsi, 1			;      we want rbp-1
; 	mov			rdx, 1			; arg: only read one byte
; 	syscall

; 	mov			rax, [rbp-1]	; store into rax for return

; 	leave
; 	ret

_exit:
	mov			rax, 60			; exit syscall: 60
	mov			rdi, rdi		; Return value: passed by rdi
	syscall

