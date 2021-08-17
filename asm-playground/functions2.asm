	global		_start

	section		.text
_start:
	mov			rdi, 0x41			; pass argument
	call		_otherfunc			; call the other function

	call		_exit				; Can't return from here: nowhere to return

_otherfunc:
	enter		0x1,0				; prepares a stack frame with 0x1 byte
	; printing stuff starts here --------------
	mov			rax, 1                  ; system call for write
	mov			rdi, 1                  ; file handle 1 is stdout
	mov			rsi, message            ; address of string to output
	mov			rdx, 13                 ; number of bytes
	syscall                             ; invoke operating system to do the write
	; printing stuff ends here ----------------
	leave							; release the stack frame
	ret								; return to the previous instruction address

; Exits the program
_exit:
	enter		0, 0				; prepares a stack frame with no room
	mov			rax, 60				; system call for exit
	mov			rdi, 0				; exit code 0
	syscall							; call into linux


	section	.data
message:
	db		"Hello, World", 10          ; note the newline at the end
