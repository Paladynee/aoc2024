extern crate alloc;
use alloc::str;
use core::fmt;
use core::fmt::Debug;

use crate::solver::SolverSentinel;
// use regex::Regex;
// #[inline]
// pub fn solve_part_1(input: &str, sentinel: &mut SolverSentinel) -> i32 {
//     // regexp approach
//     let mut regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("Corrupt regex");

//     regex
//         .captures_iter(input)
//         .map(|cap| {
//             let mulstr = cap.get(0).unwrap().as_str();
//             mulstr[4..mulstr.len() - 1]
//                 .split(',')
//                 .map(|s| s.parse::<i32>().unwrap())
//                 .fold(1i32, |mut acc, num| {
//                     acc *= num;
//                     acc
//                 })
//         })
//         .sum()
// }

#[inline]
pub fn solve_part_1(input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens();

    let mut parser = Compiler::new(tokens, Part::One);
    let nums = parser.parse_tokens();

    nums.iter().sum()
}

#[inline]
pub fn solve_part_2(input: &str, _sentinel: &mut SolverSentinel) -> i32 {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens();

    let mut parser = Compiler::new(tokens, Part::Two);
    let nums = parser.parse_tokens();

    nums.iter().sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token<'a> {
    Dont,
    Do,
    Mul,
    OpenParen,
    Num(&'a [u8]),
    Comma,
    CloseParen,
    Invalid,
    Eof,
}

struct Scanner<'a> {
    source: &'a [u8],
    tokens: Vec<Token<'a>>,
    start: usize,
    index: usize,
}

impl Debug for Scanner<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scanner")
            .field("source_len", &self.source.len())
            .field("start", &self.start)
            .field("current", &self.index)
            .finish()
    }
}

impl<'a> Scanner<'a> {
    #[inline]
    pub fn new(source: &'a str) -> Self {
        assert!(str::is_ascii(source), "cant scan non ascii stuff for now");
        Self {
            source: source.as_bytes(),
            tokens: Vec::new(),
            start: 0,
            index: 0,
        }
    }

    #[inline]
    pub const fn is_at_end(&self) -> bool {
        self.index >= self.source.len()
    }

    #[inline]
    pub fn scan_tokens(&mut self) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.index;
            if self.scan_token().is_none() {
                eprintln!("Error: {:?}", self);
            };
        }

        self.tokens.push(Token::Eof);
        &self.tokens[..]
    }

    #[inline]
    pub fn scan_token(&mut self) -> Option<()> {
        let c = self.advance();
        match c? {
            b',' => self.add_basic_token(Token::Comma),
            b'(' => self.add_basic_token(Token::OpenParen),
            b')' => self.add_basic_token(Token::CloseParen),
            b'm' => {
                if self.match1(b'u') {
                    if self.match1(b'l') {
                        self.add_basic_token(Token::Mul);
                    } else {
                        self.add_basic_token(Token::Invalid);
                    }
                } else {
                    self.add_basic_token(Token::Invalid);
                }
            }
            b'd' => {
                if self.match1(b'o') {
                    // we either expect b'(' or b'n' and we need to match on that, for that we need a peek function
                    match self.peek() {
                        Some(b'(') => {
                            self.index += 1;
                            if self.match1(b')') {
                                self.add_basic_token(Token::Do);
                            } else {
                                self.add_basic_token(Token::Invalid);
                            }
                        }
                        Some(b'n') => {
                            self.index += 1;
                            if self.match1(b'\'') {
                                if self.match1(b't') {
                                    if self.match1(b'(') {
                                        if self.match1(b')') {
                                            self.add_basic_token(Token::Dont);
                                        } else {
                                            self.add_basic_token(Token::Invalid);
                                        }
                                    } else {
                                        self.add_basic_token(Token::Invalid);
                                    }
                                } else {
                                    self.add_basic_token(Token::Invalid);
                                }
                            } else {
                                self.add_basic_token(Token::Invalid);
                            }
                        }
                        _ => self.add_basic_token(Token::Invalid),
                    }
                } else {
                    self.add_basic_token(Token::Invalid);
                }
            }

            b'0'..=b'9' => {
                self.parse_number_literal();
            }
            _ => self.add_basic_token(Token::Invalid),
        };

        Some(())
    }

    #[inline]
    pub fn advance(&mut self) -> Option<u8> {
        match self.source.get(self.index) {
            None => None,
            Some(a) => {
                self.index += 1;
                Some(*a)
            }
        }
    }

    #[inline]
    pub fn add_basic_token(&mut self, token: Token<'a>) {
        // eprint!("{:?} ", token);
        self.tokens.push(token);
    }

    #[inline]
    pub const fn match1(&mut self, exp: u8) -> bool {
        if self.is_at_end() || self.source[self.index] != exp {
            false
        } else {
            self.index += 1;
            true
        }
    }

    #[inline]
    pub const fn peek(&self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.index])
        }
    }

    #[inline]
    pub fn parse_number_literal(&mut self) -> Option<()> {
        while self.peek()?.is_ascii_digit() {
            self.advance();
        }

        self.add_basic_token(Token::Num(&self.source[self.start..self.index]));

        Some(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Part {
    One,
    Two,
}

struct Compiler<'a> {
    tokens: &'a [Token<'a>],
    outputs: Vec<i32>,
    mode: Part,
    index: usize,
    dont_fired: bool,
}

impl<'a> Compiler<'a> {
    #[inline]
    pub const fn new(tokens: &'a [Token<'a>], parser_mode_part: Part) -> Self {
        Self {
            tokens,
            index: 0,
            outputs: Vec::new(),
            dont_fired: false,
            mode: parser_mode_part,
        }
    }

    #[inline]
    pub const fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }

    #[inline]
    pub fn parse_tokens(&mut self) -> &[i32] {
        while !self.is_at_end() {
            if self.parse_token().is_none() {
                eprintln!("Error: {:?}", self);
            }
        }

        &self.outputs[..]
    }

    #[inline]
    pub fn parse_token(&mut self) -> Option<()> {
        let token = self.advance()?;
        match token {
            Token::Mul => {
                if self.match1(Token::OpenParen) {
                    if let Token::Num(lit1) = self.advance()? {
                        if self.match1(Token::Comma) {
                            if let Token::Num(lit2) = self.advance()? {
                                if self.match1(Token::CloseParen) && !self.dont_fired {
                                    let num1_lit = str::from_utf8(lit1).unwrap();
                                    let num2_lit = str::from_utf8(lit2).unwrap();

                                    let num1 = num1_lit.parse::<i32>().unwrap();
                                    let num2 = num2_lit.parse::<i32>().unwrap();

                                    self.add_num(num1 * num2);
                                }
                            }
                        }
                    }
                };
            }
            Token::Do => {
                if self.mode == Part::Two {
                    self.dont_fired = false;
                }
            }
            Token::Dont => {
                if self.mode == Part::Two {
                    self.dont_fired = true;
                }
            }
            _ => {}
        };

        Some(())
    }

    #[inline]
    pub const fn advance(&mut self) -> Option<Token<'a>> {
        if self.is_at_end() {
            None
        } else {
            let token = self.tokens[self.index];
            self.index += 1;
            Some(token)
        }
    }

    #[inline]
    pub fn match1(&mut self, exp: Token<'a>) -> bool {
        if self.is_at_end() || self.tokens[self.index] != exp {
            false
        } else {
            self.index += 1;
            true
        }
    }

    #[inline]
    pub fn add_num(&mut self, num: i32) {
        self.outputs.push(num);
    }
}

impl Debug for Compiler<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parser").field("current", &self.index).finish()
    }
}
