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

pub struct Compiler<'self> {
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

impl<'self> Compiler<'self> {
    pub fn new<'a>(pattern: &'a str) -> Compiler<'a> {
        Compiler {
            iter: pattern.char_offset_iter().peekable(),
        }
    }


    pub fn parse(&mut self) -> Result<CompiledRegexp, ~str> {
        match self.parse_fragment(None) {
            Ok(p) => {
                let mut pm = p;
                pm.push(Match);
                Ok(pm)
            },
            Err(e) => Err(e),
        }
    }

    fn parse_fragment(&mut self, delimiter: Option<char>) -> Result<CompiledRegexp, ~str> {
        let mut program = ~[];
        let mut fragment = ~[];
        loop {
            match self.parse_one() {
                Ok(p) => program = Compiler::link(program, p),
                Err(e) => return Err(e),
            };
            match self.iter.peek() {
                Some(&(_, c)) => if c == '|' && fragment.is_empty() {
                    self.iter.next();
                    fragment = program;
                    program = ~[];
                } else if c == '|' {
                    self.iter.next();
                    fragment = Compiler::link_or(fragment, program);
                    program = ~[];
                } else if delimiter.map_default(false, |&dc| dc == c) {
                    self.iter.next();
                    break;
                },
                None => break,
            };
        }

        if fragment.is_empty() {
            Ok(program)
        } else {
            Ok(Compiler::link_or(fragment, program))
        }
    }

    fn link(p1: CompiledRegexp, p2: CompiledRegexp) -> CompiledRegexp {
        let len = p1.len();
        let mut pm = p2;
        for i in range(0, pm.len()) {
            match pm[i] {
                Split(a, b) => pm[i] = Split(len+a, len+b),
                Jmp(a) => pm[i] = Jmp(len+a),
                _ => {},
            }
        }
        vec::append(p1, pm)
    }

    fn link_or(p1: CompiledRegexp, p2: CompiledRegexp) -> CompiledRegexp {
        let len1 = p1.len();
        let len2 = p2.len();
        let mut pm = p1;
        pm = Compiler::link(~[Split(1, len1+2)], pm);
        pm.push(Jmp(len1+len2+2));
        Compiler::link(pm, p2)
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
        self.parse_fragment(Some(')'))
    }
}

pub fn compile(_pattern: &str) -> Vm {
    Vm::new()
}

fn main() {
    // let s = ~"a?b+c*|d*|e+";
    // let s = ~"a+b+|a+b+";
    let s = ~"(ab)+";
    let mut p = Compiler::new(s);
    printfln!(p.parse());
}
