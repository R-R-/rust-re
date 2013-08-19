use std::bool;
use std::vec;

use parse;

enum IterResult {
    Matched,
    Continue,
    Halt,
}

pub struct Vm {
    program: parse::CompiledRegexp,
    ips: ~[uint],
}

impl Vm {
    pub fn new(program: parse::CompiledRegexp) -> Vm {
        Vm {
            program: program,
            ips: ~[],
        }
    }

    pub fn matches(&mut self, string: &str) -> bool {
        let mut iter = string.char_offset_iter();
        for _ in range(0, string.char_len()) {
            self.init();
            for (_, c) in iter.clone() {
                match self.iterate(c) {
                    Matched => return true,
                    Halt => break,
                    _ => {},
                }
            }
            for addr in self.ips.iter() {
                match self.program[*addr] {
                    parse::Match => return true,
                    _ => {},
                }
            }
            iter.next();
        }
        false
    }

    fn init(&mut self) {
        self.ips = self.follow_jump(0);
        if self.ips.is_empty() {
            self.ips.push(0);
        }
    }

    fn iterate(&mut self, c: char) -> IterResult {
        if self.ips.is_empty() {
            return Halt;
        } else {
            let mut new_ips = ~[];
            let mut result = Continue;
            for addr in self.ips.iter() {
                match self.program[*addr] {
                    parse::Char(ch) => if ch == c {
                        let new_addrs = self.follow_jump(*addr+1);
                        if new_addrs.is_empty() {
                            new_ips.push(*addr+1);
                        } else {
                            new_ips = vec::append(new_ips, new_addrs);
                        }
                    },
                    parse::Match => result = Matched,
                    _ => fail!("Unexpected jump instruction."),
                }
            }
            self.ips = new_ips;
            result
        }
    }

    fn follow_jump(&self, i: uint) -> ~[uint] {
        let mut addresses = ~[];
        let mut working_set = ~[i];
        while bool::not(working_set.is_empty()) {
            let mut new_working_set = ~[];
            for address in working_set.iter() {
                match self.program[*address] {
                    parse::Split(a, b) => {
                        new_working_set.push(a);
                        new_working_set.push(b);
                    },
                    parse::Jmp(a) => new_working_set.push(a),
                    _ => addresses.push(*address),
                }
            }
            working_set = new_working_set;
        }
        addresses
    }
}

pub fn compile(pattern: &str) -> Result<Vm, ~str> {
    let mut compiler = parse::Compiler::new(pattern);
    match compiler.compile() {
        Ok(p) => Ok(Vm::new(p)),
        Err(e) => Err(e),
    }
}
