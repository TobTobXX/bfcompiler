use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::{File, write};
use std::process::Command as SysCommand;

fn main() -> io::Result<()> {
    let f = File::open("fixture.bf")?;
    let f = BufReader::new(f);

    let mut nesting_level = 0;

    let mut id_stack: Vec<u16> = vec![];
    let mut id_generator: u16 = 0;

    let content: String = f.lines().map(Result::unwrap).collect();

    let asm: String = content.chars()
        .filter(|c| {
            match c {
                '+' | '-' | '<' | '>' | '[' | ']' | ',' | '.' => true,
                _ => false,
            }
        })
        .map(|c| {
            use Command::*;
            match c {
                '+' => IncrementMem,
                '-' => DecrementMem,
                '<' => DecrementPtr,
                '>' => IncrementPtr,
                '[' => {
                    nesting_level += 1;
                    JumpIfZero(nesting_level)
                },
                ']' => {
                    nesting_level -= 1;
                    JumpBack(nesting_level + 1)
                }
                ',' => Input,
                '.' => Output,
                _ => panic!(),
            }
        })
        .map(|c| {
            c.to_asm(&mut id_stack, &mut id_generator)
        })
        .collect();

    let asm = format!("
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
    enter       8192, 0
    mov         r15, rbp
    sub         r15, 8
    {}
    leave
    ret
", asm);

    write("out.asm", asm)?;

    SysCommand::new("nasm")
            .args(&["-f elf64", "-o ir.o", "out.asm"])
            .output()?;
    SysCommand::new("ld")
            .args(&["-oa.out", "ir.o"])
            .output()?;
    SysCommand::new("chmod")
            .args(&["+x", "a.out"])
            .output()?;

    Ok(())
}

#[derive(Debug)]
enum Command {
    IncrementMem,
    DecrementMem,
    DecrementPtr,
    IncrementPtr,
    JumpIfZero(u16),
    JumpBack(u16),
    Input,
    Output,
}

impl Command {
    fn to_asm(self, id_stack: &mut Vec<u16>, id_generator: &mut u16) -> String {
        use Command::*;
        match self {
            IncrementMem =>
                "inc DWORD [r15]\n".to_string(),
            DecrementMem =>
                "dec DWORD [r15]\n".to_string(),
            DecrementPtr =>
                "add r15, 4\n".to_string(),
            IncrementPtr =>
                "sub r15, 4\n".to_string(),
            JumpIfZero(_) => {
                id_stack.push(*id_generator);
                let lbl = format!("opening_lbl{}:", *id_generator);
                let instr1 = "cmp DWORD [r15], 0";
                let instr2 = format!("jz closing_lbl{}", *id_generator);
                *id_generator += 1;
                format!("{}\n{}\n{}\n", lbl, instr1, instr2)
            },
            JumpBack(_) => {
                let prev_lbl = id_stack.pop().unwrap();
                let instr = format!("jmp opening_lbl{}\n", prev_lbl);
                let lbl = format!("closing_lbl{}:", prev_lbl);
                format!("{}\n{}\n", instr, lbl)
            },
            Input =>
                "call _input\n".to_string(),
            Output =>
                "call _output\n".to_string(),
        }
    }
}

