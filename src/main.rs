use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::{File, write};
use std::process::Command as SysCommand;
use std::env::args;

fn main() -> io::Result<()> {
    let config = Config::parse_args(&(args().collect::<Vec<String>>()));
    
    let f = File::open(config.input_file)?;
    let f = BufReader::new(f);

    let mut nesting_level = 0;

    let mut id_stack: Vec<u16> = vec![];
    let mut id_generator: u16 = 0;

    let content: String = f.lines().map(Result::unwrap).collect();

    println!("Compiling...");

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
    enter       {}, 0
    mov         r15, rbp
    sub         r15, 8
    {}
    leave
    ret
", config.bufsize, asm);

    write("out.asm",asm)?;

    println!("Assembling...");

    if config.optimisation {
	SysCommand::new("nasm")
            .args(&["-f elf64", "-o ir.o", "out.asm"])
            .output()?;
    } else {
	SysCommand::new("nasm")
            .args(&["-f elf64", "-o ir.o", "out.asm", "-O0"])
            .output()?;
    }

    println!("Linking...");
    
    SysCommand::new("ld")
            .args(&[format!("-o{}", config.output_file).as_str(), "ir.o"])
            .output()?;
    SysCommand::new("chmod")
            .args(&["+x", config.output_file.as_str()])
            .output()?;

    println!("Done!");
    
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

struct Config {
    pub input_file: String,
    pub output_file: String,
    pub optimisation: bool,
    pub bufsize: usize,
}

impl Config {
    pub fn parse_args(args: &[String]) -> Self {
	let mut config = Self{
	    input_file: String::new(),
	    output_file: "a.out".to_string(),
	    optimisation: false,
	    bufsize: 512
	};
	
	for arg in args {
	    let (earg, param) = if arg.contains('=') {
		let parts = arg.split('=').map(|a| a.to_string()).collect::<Vec<String>>();
		(parts[0].to_string(), parts[1].to_string())
	    } else {
		(arg.to_string(), "".to_string())
	    };
	    match earg.as_str() {
		"--optimized" | "-O" => config.optimisation = true,
		"--input" | "-i" => config.input_file = param,
		"--output" | "-o" => config.output_file = param,
		"--bufsize" | "-b" => config.bufsize = param.parse().unwrap(),
		_ => {},
	    }
	}

	if config.input_file.is_empty() {
	    println!("No input file specified.");
	    std::process::exit(-1);
	}
	
	config
    }
}

#[test]
fn test_parse_config() {
    let test = vec!["--bufsize=8192", "--optimized", "--input=hello.asm"]
	.iter()
	.map(|e| e.to_string())
	.collect::<Vec<String>>();
    let res = Config::parse_args(&test);
    assert_eq!(res.bufsize, 8192);
    assert!(res.optimisation);
    assert_eq!(res.input_file, "hello.asm".to_string());
}
