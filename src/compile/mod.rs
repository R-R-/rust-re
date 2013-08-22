use std::iterator;
use std::str;
use std::vec;

type Iter<'self> = iterator::Peekable<(uint, char), str::CharOffsetIterator<'self>>;

/// All the instructions that the virtual machine understands
#[deriving(Clone)]
pub enum Instruction {
    /// match one character
    Char(char),
    /// match any char
    Dot,
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

pub struct Compiler<'self> {
    iter: Iter<'self>,
}

static UNEXPECTED_EOS: &'static str = "Unexpected end of stream.";

impl<'self> Compiler<'self> {
    pub fn new<'a>(pattern: &'a str) -> Compiler<'a> {
        Compiler {
            iter: pattern.char_offset_iter().peekable(),
        }
    }

    pub fn compile(&mut self) -> Result<CompiledRegexp, ~str> {
        match self.compile_fragment(None) {
            Ok((p, _)) => {
                let mut pm = p;
                pm.push(Match);
                Ok(pm)
            },
            Err(e) => Err(e),
        }
    }

    fn compile_fragment(&mut self, delimiter: Option<char>)
        -> Result<(CompiledRegexp, bool), ~str> {
        let mut program = ~[];
        let mut fragment = ~[];
        let mut found_delimiter = false;
        loop {
            match self.compile_one() {
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
                    found_delimiter = true;
                    break;
                },
                None => break,
            };
        }

        if fragment.is_empty() {
            Ok((program, found_delimiter))
        } else {
            Ok((Compiler::link_or(fragment, program), found_delimiter))
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

    fn compile_one(&mut self) -> Result<CompiledRegexp, ~str> {
        let mut program = ~[];
        match self.iter.next() {
            Some((i, c)) => match c {
                '?' | '*' | '+' | ')' | '|' =>
                    return Err(fmt!("Unexpected char '%c' at %u", c, i)),
                '(' => match self.compile_group() {
                    Ok(p) => program = p,
                    Err(e) => return Err(e),
                },
                '.' => program.push(Dot),
                '\\' => match self.iter.next() {
                    Some((_, c)) => program.push(Char(c)),
                    None => return Err(UNEXPECTED_EOS.to_owned()),
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
                        program = Compiler::link(~[Split(1, len+1)], program);
                        self.iter.next();
                    },
                    '*' => {
                        program = Compiler::link(~[Split(1, len+2)], program);
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

    fn compile_group(&mut self) -> Result<CompiledRegexp, ~str> {
        match self.compile_fragment(Some(')')) {
            Ok((p, found_delimiter)) => if found_delimiter {
                Ok(p)
            } else {
                Err(UNEXPECTED_EOS.to_owned())
            },
            Err(e) => Err(e),
        }
    }
}
