use std::convert::Infallible;

use clap::builder::{ValueParser, ValueParserFactory};
use nom::{
    bytes::complete::{is_not, tag},
    combinator::{map_res, opt},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::Node;

#[derive(Debug, Clone)]
pub struct Brackets {
    open: String,
    close: String,
}
impl ValueParserFactory for Brackets {
    type Parser = ValueParser;

    fn value_parser() -> Self::Parser {
        ValueParser::new(|string: &str| -> Result<Self, &'static str> {
            let chars = string.chars().collect::<Vec<_>>();
            let halflen = string.len() / 2;
            if halflen == 0 {
                return Err("Not enough chars in string");
            }

            let open = chars[..halflen].iter().collect();
            let close = chars[halflen..].iter().collect();

            Ok(Brackets { open, close })
        })
    }
}

#[derive(Debug, Clone)]
pub struct ParseMode {
    delimiter: String,
    brackets: Brackets,
    trim: bool,
}
impl ParseMode {
    pub fn new(delimiter: String, brackets: Brackets, trim: bool) -> Self {
        ParseMode {
            delimiter,
            brackets,
            trim,
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
            map_node(mode.trim),
        )(i)
    }
}
fn bracketed(mode: ParseMode) -> impl Fn(&str) -> IResult<&str, Vec<Node>> {
    move |i: &str| {
        delimited(
            tag(mode.brackets.open.as_str()),
            node_list(mode.clone()),
            tag(mode.brackets.close.as_str()),
        )(i)
    }
}
fn node_list(mode: ParseMode) -> impl Fn(&str) -> IResult<&str, Vec<Node>> {
    move |i: &str| separated_list0(tag(mode.delimiter.as_str()), single_node(mode.clone()))(i)
}
fn map_node(trim: bool) -> impl Fn((&str, Option<Vec<Node>>)) -> Result<Node, Infallible> {
    move |(name, children)| {
        Ok(Node::new(
            match trim {
                true => name.trim().to_owned(),
                false => name.to_owned(),
            },
            children.unwrap_or_default(),
        ))
    }
}
