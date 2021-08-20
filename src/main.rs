use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("fixture.bf")?;
    let f = BufReader::new(f);

    let mut nesting_level = 0;

    for line in f.lines() {
        let line = line.unwrap();
        line.chars()
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
            .for_each(|cmd| println!("{:?}", cmd));
    }

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

trait ToAsm {
    fn to_asm(self) -> String;
}

impl ToAsm for Command {
    fn to_asm(self) -> String {
        use Command::*;
        match self {
            IncrementMem => {
                concat!(
                    "",
                    "",
                    ""
                ).to_owned()
                // "mov ".to_string() + 
                // "".to_string()
            }
            _ => unimplemented!(),
        }
    }
}

