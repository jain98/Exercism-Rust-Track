#![allow(unused)]

#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;
use std::borrow::Cow;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    custom_op_map: HashMap<String, String>
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug)]
enum Op {
    ADD,
    SUB,
    MUL,
    DIV,
    DUP,
    DROP,
    SWAP,
    OVER,
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        match &token.to_ascii_lowercase()[..] {
            "+" => Ok(Op::ADD),
            "-" => Ok(Op::SUB),
            "*" => Ok(Op::MUL),
            "/" => Ok(Op::DIV),
            "dup" => Ok(Op::DUP),
            "drop" => Ok(Op::DROP),
            "swap" => Ok(Op::SWAP),
            "over" => Ok(Op::OVER),
            _ => {
                Err(Error::UnknownWord)
            }
        }
    }
}

impl Op {
    fn double_operand_op(&self, operand1: i32, operand2: i32) -> Result<Vec<i32>, Error> {
        match self {
            Op::ADD => Ok(vec![operand1+operand2]),
            Op::SUB => Ok(vec![operand1-operand2]),
            Op::MUL => Ok(vec![operand1*operand2]),
            Op::DIV => if operand2 == 0 { Err(Error::DivisionByZero) } else { Ok(vec![operand1/operand2]) }
            Op::SWAP => Ok(vec![operand2, operand1]),
            Op::OVER => Ok(vec![operand1, operand2, operand1]),
            Op::DUP | Op::DROP => panic!("Cannot perform double operand operations for Op::{:?}", self)
        }
    }

    fn single_operand_op(&self, operand1: i32) -> Vec<i32> {
        match self {
            Op::DUP => vec![operand1, operand1],
            Op::DROP => vec![],
            _ => panic!("Single operand operations only implemented for DROP & DUP!")
        }
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            custom_op_map: HashMap::new()
        }
    }

    pub fn stack(&self) -> &[Value] {
       &self.stack 
    }

    pub fn eval(&mut self, orig_input: &str) -> ForthResult {
        // Find out if input contains any custom op definitions?
        // - If it does try and define them and remove the definitions from input
        // - If error occurs while defining them throw error
        let input = &*self.replace_custom_ops_definitions(orig_input)?;

        let mut basic_input = input.to_ascii_lowercase();
        for (k, v) in &self.custom_op_map {
            if !basic_input.is_empty() {
                basic_input = basic_input.replace(k, v);
            }
        }

        for token in basic_input.split_ascii_whitespace() {
            if token == ":" {
                return Err(Error::InvalidWord);
            } else if let Ok(n) = token.parse::<i32>() {
                self.stack.push(n);
            } else if let Ok(op) = token.parse::<Op>() {
                self.peform_op(op)?;
            } else{
                // At this point we'd have ruled out any invalid words, so
                // if we get a word we don't recognize we should be throwing
                // unknown word error
                return Err(Error::UnknownWord);
            }
        }

        Ok(())
    }

    fn replace_custom_ops_definitions<'a>(&mut self, input: &'a str) -> Result<Cow<'a, str>, Error> {
        // Compiling the same regular expression with each method call is expensive and
        // should be done exactly once
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r": (?P<name>[[[:alpha:]][[:punct:]]]+) (?P<def>[[[:alnum:]][+\-*/]\s*]+) ;").unwrap();
        }

        for capture in PATTERN.captures_iter(input) {
            // Recursively replace as many tokens in the captured definition as possible
            let def = self.replace_existing_ops_from_custom_ops_definition(capture["def"].to_owned())?;
            self.custom_op_map.insert(capture["name"].to_ascii_lowercase(), def.to_ascii_lowercase());
        }

        Ok(PATTERN.replace_all(input, ""))
    }

    fn replace_existing_ops_from_custom_ops_definition(&self, def: String) -> Result<String, Error> {
        let mut result = vec![];
        for token in def.split_ascii_whitespace() {
            // Verify that token is either a valid Op - either pre-defined or custom
            self.verify_token_is_valid(token);

            let mut curr = token.to_owned();
            while let Some(value) = self.custom_op_map.get(&curr) {
                curr = self.replace_existing_ops_from_custom_ops_definition(value.clone())?;
            }
            result.push(curr);
        }
        Ok(result.join(" "))
    }

    fn verify_token_is_valid(&self, token: &str) -> Result<bool, Error> {
        return if let Ok(n) = token.parse::<i32>() {
            Ok(true)
        } else if let Ok(op) = token.parse::<Op>() {
            Ok(true)
        } else if self.custom_op_map.contains_key(token) {
            Ok(true)
        } else {
            Err(Error::InvalidWord)
        }
    }

    fn peform_op(&mut self, op: Op) -> ForthResult {
        match op {
            Op::ADD | Op::SUB | Op::MUL | Op::DIV | Op::SWAP | Op::OVER => {
                let operand2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let operand1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let result = op.double_operand_op(operand1, operand2)?;
                self.stack.extend(result);
                Ok(())
            },
            Op::DUP | Op::DROP => {
                let operand1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.extend(op.single_operand_op(operand1));
                Ok(())
            }
        }
    }

    // fn perform_custom_op(&mut self, token: &str) -> ForthResult {
    //
    // }

    // fn parse_token(token: &str) -> Result<Op, Error> {
    //     Op::from_str(token)
    // }
}
