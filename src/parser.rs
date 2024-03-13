use std::convert::Infallible;

use clap::builder::{ValueParser, ValueParserFactory};
use nom::{
    bytes::complete::is_not,
    character::complete::char,
    combinator::{map_res, opt},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::Node;

#[derive(Debug, Clone)]
pub struct Brackets {
    open: char,
    close: char,
}
impl ValueParserFactory for Brackets {
    type Parser = ValueParser;

    fn value_parser() -> Self::Parser {
        ValueParser::new(|string: &str| -> Result<Self, &'static str> {
            let mut chars = string.chars();
            let brackets = Brackets {
                open: chars.next().ok_or("Not enough chars in string")?,
                close: chars.next().ok_or("Not enough chars in string")?,
            };

            match chars.next() {
                None => Ok(brackets),
                Some(_) => Err("Too many chars in string"),
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct ParseMode {
    delimiter: char,
    brackets: Brackets,
}
impl ParseMode {
    pub fn new(delimiter: char, brackets: Brackets) -> Self {
        ParseMode {
            delimiter,
            brackets,
        }
    }
}
impl ToString for ParseMode {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.delimiter, self.brackets.open, self.brackets.close
        )
    }
}

pub fn parse_nodes(input: &str, mode: ParseMode) -> IResult<&str, Vec<Node>> {
    node_list(mode)(input)
}

fn single_node(mode: ParseMode) -> impl Fn(&str) -> IResult<&str, Node> {
    move |i: &str| {
        map_res(
            tuple((
                is_not(mode.to_string().as_str()),
                opt(bracketed(mode.clone())),
            )),
            map_node,
        )(i)
    }
}
fn bracketed(mode: ParseMode) -> impl Fn(&str) -> IResult<&str, Vec<Node>> {
    move |i: &str| {
        delimited(
            char(mode.brackets.open),
            node_list(mode.clone()),
            char(mode.brackets.close),
        )(i)
    }
}
fn node_list(mode: ParseMode) -> impl Fn(&str) -> IResult<&str, Vec<Node>> {
    move |i: &str| separated_list0(char(mode.delimiter), single_node(mode.clone()))(i)
}
fn map_node((name, children): (&str, Option<Vec<Node>>)) -> Result<Node, Infallible> {
    Ok(Node::new(name.to_owned(), children.unwrap_or_default()))
}
