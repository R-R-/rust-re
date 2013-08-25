use std::bool;
use std::vec;

use compile;
use compile::inst;

enum IterResult {
    Matched,
    Continue,
    Halt,
}

pub struct Engine {
    program: compile::CompiledRegexp,
    ips: ~[uint],
}

impl Engine {
    pub fn new(program: compile::CompiledRegexp) -> Engine {
        Engine {
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
                    inst::Succeed => return true,
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
                    inst::Match(m) => match m {
                        inst::Char(ch) => if ch == c {
                            new_ips = vec::append(new_ips, self.follow_jump(*addr+1));
                        },
                        inst::Dot => new_ips = vec::append(new_ips, self.follow_jump(*addr+1)),
                    },
                    inst::Succeed => result = Matched,
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
                    inst::Split(a, b) => {
                        new_working_set.push(a);
                        new_working_set.push(b);
                    },
                    inst::Jmp(a) => new_working_set.push(a),
                    _ => addresses.push(*address),
                }
            }
            working_set = new_working_set;
        }
        addresses
    }
}

pub fn compile(pattern: &str) -> Result<Engine, ~str> {
    match compile::compile(pattern) {
        Ok(p) => Ok(Engine::new(p)),
        Err(e) => Err(e),
    }
}
