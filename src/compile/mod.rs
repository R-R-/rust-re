use std::vec;

mod parse;

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

pub fn compile(pattern: &str) -> Result<CompiledRegexp, ~str> {
    let mut parser = parse::Parser::new(pattern);
    let mut compiler = Compiler::new();
    match parser.parse() {
        Ok(ast) => {
            compiler.compile(ast);
            match compiler {
                Compiler(r) => Ok(r),
            }
        }
        Err(e) => Err(e),
    }
}

struct Compiler(CompiledRegexp);

impl Compiler {
    pub fn new() -> Compiler {
        Compiler(~[])
    }

    pub fn compile(&mut self, ast: &[parse::Ast]) {
        self.compile_internal(ast);
        self.push(Match);
    }

    fn compile_internal(&mut self, ast: &[parse::Ast]) {
        for fragment in ast.iter() {
            match fragment {
                &parse::Fragment(ref one, ref modifier) => self.compile_fragment(one, modifier),
                &parse::Or(ref asts) => {
                    let mut jmps = ~[];
                    for a in asts.iter() {
                        let idx = self.len();
                        self.push(Jmp(-1));
                        self.compile_internal(*a);
                        self.push(Jmp(-1));
                        let l1 = idx + 1;
                        let l2 = self.len();
                        self[idx] = Split(l1, l2);
                        jmps.push(l2 - 1);
                    }
                    let len = self.len();
                    for jmp in jmps.iter() {
                        self[*jmp] = Jmp(len);
                    }
                },
            }
        }
    }

    fn compile_fragment(&mut self, one: &parse::One, modifier: &parse::Modifier) {
        match modifier {
            &parse::No => self.compile_one(one),
            &parse::QMark => {
                let idx = self.len();
                let l1 = idx + 1;
                self.push(Jmp(-1));
                self.compile_one(one);
                let l2 = self.len();
                self[idx] = Split(l1, l2);
            },
            &parse::Star => {
                let idx = self.len();
                let l1 = idx;
                let l2 = idx + 1;
                self.push(Jmp(-1));
                self.compile_one(one);
                let l3 = self.len() + 1;
                self[idx] = Split(l2, l3);
                self.push(Jmp(l1));
            },
            &parse::Plus => {
                let l1 = self.len();
                self.compile_one(one);
                let l2 = self.len() + 1;
                self.push(Split(l1, l2));
            },
        }
    }

    fn compile_one(&mut self, one: &parse::One) {
        match one {
            &parse::Char(c) => self.push(Char(c)),
            &parse::Dot => self.push(Dot),
            &parse::Group(ref ast) => self.compile_internal(*ast),
        }
    }
}
