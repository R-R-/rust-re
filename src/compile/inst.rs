/// All the instructions that the virtual machine understands
#[deriving(Clone)]
pub enum Instruction {
    /// simple match
    Match(Match),
    /// unconditional jump
    Jmp(uint),
    /// successful match
    Succeed,
    /// split current virtual thread into two
    Split(uint, uint),
}

/// Instructions denoting simple matches
#[deriving(Clone)]
pub enum Match {
    /// match one character
    Char(char),
    /// match any char
    Dot,
}
