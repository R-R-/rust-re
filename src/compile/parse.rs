use std::iterator;
use std::str;

use compile::inst;

pub static UNEXPECTED_EOS: &'static str = "Unexpected end of stream.";

pub enum One {
    Match(inst::Match),
    Group(~[Ast]),
}

pub enum Modifier {
    No,
    Plus,
    QMark,
    Star,
}

pub enum Ast {
    Or(~[~[Ast]]),
    Fragment(One, Modifier),
}

pub type Iter<'self> = iterator::Peekable<(uint, char), str::CharOffsetIterator<'self>>;

pub struct Parser<'self> {
    iter: Iter<'self>,
}

impl<'self> Parser<'self> {
    pub fn new<'a>(pattern: &'a str) -> Parser<'a> {
        Parser {
            iter: pattern.char_offset_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<~[Ast], ~str> {
        match self.parse_fragment(None) {
            Ok((ast, _)) => Ok(ast),
            Err(e) => Err(e),
        }
    }

    pub fn parse_fragment(&mut self, delimiter: Option<char>) -> Result<(~[Ast], bool), ~str> {
        let mut fragment = ~[];
        let mut ast = ~[];
        let mut found_delimiter = false;
        loop {
            match self.parse_one() {
                Ok(o) => match o {
                    Some(p) => ast.push(p),
                    None => break,
                },
                Err(e) => return Err(e),
            };
            match self.iter.peek() {
                Some(&(_, c)) => match c {
                    '|' => {
                        self.iter.next();
                        fragment.push(ast);
                        ast = ~[];
                    },
                    _ if delimiter.map_default(false, |&dc| dc == c) => {
                        self.iter.next();
                        found_delimiter = true;
                        break;
                    },
                    _ => (),
                },
                None => break,
            };
        }

        if fragment.is_empty() {
            Ok((ast, found_delimiter))
        } else {
            Ok((~[Or(fragment)], found_delimiter))
        }
    }

    fn parse_one(&mut self) -> Result<Option<Ast>, ~str> {
        let mut one: One;
        let mut modifier: Modifier;
        match self.iter.next() {
            Some((i, c)) => match c {
                '?' | '*' | '+' | ')' | '|' =>
                    return Err(fmt!("Unexpected char '%c' at %u", c, i)),
                '(' => match self.parse_group() {
                    Ok(p) => one = Group(p),
                    Err(e) => return Err(e),
                },
                '.' => one = Match(inst::Dot),
                '\\' => match self.iter.next() {
                    Some((_, c)) => one = Match(inst::Char(c)),
                    None => return Err(UNEXPECTED_EOS.to_owned()),
                },
                _ => one = Match(inst::Char(c)),
            },
            None => return Ok(None),
        };
        match self.iter.peek() {
            Some(&(_, ch)) => {
                match ch {
                    '?' => {
                        modifier = QMark;
                        self.iter.next();
                    },
                    '*' => {
                        modifier = Star;
                        self.iter.next();
                    },
                    '+' => {
                        modifier = Plus;
                        self.iter.next();
                    },
                    _ => modifier = No,
                }
            },
            None => modifier = No,
        };
        Ok(Some(Fragment(one, modifier)))
    }

    fn parse_group(&mut self) -> Result<~[Ast], ~str> {
        match self.parse_fragment(Some(')')) {
            Ok((p, found_delimiter)) => if found_delimiter {
                Ok(p)
            } else {
                Err(UNEXPECTED_EOS.to_owned())
            },
            Err(e) => Err(e),
        }
    }
}
