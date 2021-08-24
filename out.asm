
	global		_start

	section		.text
_start:
	enter		0, 0

    call _code

	jmp			_exit

_output:
	enter		0, 0
	mov			rax, 1				; syscall number: 1 for write
	mov			rdi, 1				; arg: file descriptor: 1 is stdout
	mov			rsi, r15			; arg: prointer to buffer
	mov			rdx, 1				; arg: only write one byte
	syscall
	leave
	ret

_input:
	enter		0, 0
	mov			rax, 0				; syscall number: 0 for read
	mov			rdi, 0				; arg: file descriptor: 0 is stdin
	mov			rsi, r15			; arg: prointer to buffer
	mov			rdx, 1				; arg: only read one byte
	syscall

	leave
	ret

_exit:
	mov			rax, 60				; exit syscall: 60
	mov			rdi, 0				; Return value: 0
	syscall

_code: 
    enter       512, 0
    mov         r15, rbp
    sub         r15, 8
    opening_lbl0:
cmp DWORD [r15], 0
jz closing_lbl0
call _input
call _output
opening_lbl1:
cmp DWORD [r15], 0
jz closing_lbl1
call _output
jmp opening_lbl1

closing_lbl1:
call _input
call _output
call _output
call _input
call _input
call _input
inc DWORD [r15]
call _input
dec DWORD [r15]
call _input
add r15, 4
sub r15, 4
call _input
opening_lbl2:
cmp DWORD [r15], 0
jz closing_lbl2
jmp opening_lbl2

closing_lbl2:
call _output
call _output
jmp opening_lbl0

closing_lbl0:
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
opening_lbl3:
cmp DWORD [r15], 0
jz closing_lbl3
sub r15, 4
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
opening_lbl4:
cmp DWORD [r15], 0
jz closing_lbl4
sub r15, 4
inc DWORD [r15]
inc DWORD [r15]
sub r15, 4
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
sub r15, 4
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
sub r15, 4
inc DWORD [r15]
add r15, 4
add r15, 4
add r15, 4
add r15, 4
dec DWORD [r15]
jmp opening_lbl4

closing_lbl4:
sub r15, 4
inc DWORD [r15]
sub r15, 4
inc DWORD [r15]
sub r15, 4
dec DWORD [r15]
sub r15, 4
sub r15, 4
inc DWORD [r15]
opening_lbl5:
cmp DWORD [r15], 0
jz closing_lbl5
add r15, 4
jmp opening_lbl5

closing_lbl5:
add r15, 4
dec DWORD [r15]
jmp opening_lbl3

closing_lbl3:
sub r15, 4
sub r15, 4
call _output
sub r15, 4
dec DWORD [r15]
dec DWORD [r15]
dec DWORD [r15]
call _output
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
call _output
call _output
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
call _output
sub r15, 4
sub r15, 4
call _output
add r15, 4
dec DWORD [r15]
call _output
add r15, 4
call _output
inc DWORD [r15]
inc DWORD [r15]
inc DWORD [r15]
call _output
dec DWORD [r15]
dec DWORD [r15]
dec DWORD [r15]
dec DWORD [r15]
dec DWORD [r15]
dec DWORD [r15]
call _output
dec DWORD [r15]
dec DWORD [r15]
dec DWORD [r15]
dec DWORD [r15]
dec DWORD [r15]
dec DWORD [r15]
dec DWORD [r15]
dec DWORD [r15]
call _output
sub r15, 4
sub r15, 4
inc DWORD [r15]
call _output
sub r15, 4
inc DWORD [r15]
inc DWORD [r15]
call _output

    leave
    ret
