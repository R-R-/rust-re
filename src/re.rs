use std::iterator;
use std::str;
use std::vec;

static UNEXPECTED_EOS: &'static str = "Unexpected end of stream.";

/// All the instructions that the virtual machine understands
#[deriving(Clone)]
pub enum Instruction {
    /// match one character
    Char(char),
    /// unconditional jump
    Jmp(uint),
    /// successful match
    Match,
    /// split current virtual thread into two
    Split(uint, uint),
}

/// Compiled version of a regular expression,
/// to be executed by a virtual machine
pub type CompiledRegexp = ~[Instruction];

/// Virtual machine to execute a compiled regular expression
pub struct Vm {
    program: CompiledRegexp,
    ips: ~[uint],
}

type Iter<'self> = iterator::Peekable<(uint, char), str::CharOffsetIterator<'self>>;

pub struct Parser<'self> {
    iter: Iter<'self>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            program: ~[],
            ips: ~[],
        }
    }
}

impl<'self> Parser<'self> {
    pub fn new<'a>(pattern: &'a str) -> Parser<'a> {
        Parser {
            iter: pattern.char_offset_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<CompiledRegexp, ~str> {
        let mut program = ~[];
        loop {
            match self.parse_one() {
                Ok(p) => program = Parser::link(program, p),
                Err(e) => return Err(e),
            };
            match self.iter.peek() {
                Some(&(_, c)) => if c == '|' {
                    self.iter.next();
                },
                None => break,
            };
        }
        program.push(Match);
        Ok(program)
    }

    fn link(p1: ~[Instruction], p2: ~[Instruction]) -> CompiledRegexp {
        let len = p1.len();
        let mut pr = p2;
        for i in range(0, pr.len()) {
            match pr[i] {
                Split(a, b) => pr[i] = Split(len+a, len+b),
                Jmp(a) => pr[i] = Jmp(len+a),
                _ => {},
            }
        }
        vec::append(p1, pr)
    }

    fn parse_one(&mut self) -> Result<CompiledRegexp, ~str> {
        let mut program = ~[];
        match self.iter.next() {
            Some((i, c)) => match c {
                '?' | '*' | '+' | ')' | '|' =>
                    return Err(fmt!("Unexpected char '%c' at %u.", c, i)),
                '(' => match self.parse_group() {
                    Ok(p) => program = p,
                    Err(e) => return Err(e),
                },
                _ => program.push(Char(c)),
            },
            None => return Ok(program),
        };
        let len = program.len();
        match self.iter.peek() {
            Some(&(_, ch)) => {
                match ch {
                    '?' => {
                        program = vec::append(~[Split(1, len+1)], program);
                        self.iter.next();
                    },
                    '*' => {
                        program = vec::append(~[Split(1, len+2)], program);
                        program.push(Jmp(0));
                        self.iter.next();
                    },
                    '+' => {
                        program.push(Split(0, len+1));
                        self.iter.next();
                    },
                    _ => {},
                }
            },
            None => {},
        };
        Ok(program)
    }

    fn parse_group(&mut self) -> Result<CompiledRegexp, ~str> {
        Ok(~[])
    }
}

pub fn compile(_pattern: &str) -> Vm {
    Vm::new()
}

fn main() {
    // let s = ~"a?b+c*|d*|e+";
    let s = ~"a+b+";
    let mut p = Parser::new(s);
    printfln!(p.parse());
}
