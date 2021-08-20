	global		_start

	section		.text
_start:
	enter		512, 0				; 512(memory) + 4(pointer)

	mov			r15, rbp
	sub			r15, 8

	inc			DWORD [r15]

	; mov			rax, [rbp-8]
	; inc			rax
	; call		_input
	; mov			[rbp-1], rax
	; mov			rax, [rbp-1]
	; call		_output

	jmp			_exit

; Output the char in rax to stdout
_output:
	enter		1, 0
	mov			[rbp-1], rax		; save rax to stack
	mov			rax, 1				; syscall number: 1 for write
	mov			rdi, 1				; arg: file descriptor: 1 is stdout
	mov			rsi, rbp			; arg: prointer to buffer
	sub			rsi, 1				;      we want rbp-1
	mov			rdx, 1				; arg: only write one byte
	syscall
	leave
	ret

; Blocking on one char input and storing it into rax
; Note: To flush the stdin, the terminal will have to receive a newline
_input:
	enter		1, 0
	mov			rax, 0				; syscall number: 0 for read
	mov			rdi, 0				; arg: file descriptor: 0 is stdin
	mov			rsi, rbp			; arg: prointer to buffer
	sub			rsi, 1				;      we want rbp-1
	mov			rdx, 1				; arg: only read one byte
	syscall

	mov			rax, [rbp-1]		; store into rax for return

	leave
	ret

_exit:
	mov			rax, 60				; exit syscall: 60
	mov			rdi, 0				; Return value: 0
	syscall

