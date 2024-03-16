use std::convert::Infallible;

use clap::builder::{ValueParser, ValueParserFactory};
use nom::{
    bytes::complete::{tag, take_while},
    combinator::{map_res, opt},
    error::ErrorKind,
    multi::separated_list0,
    sequence::{delimited, tuple},
    FindSubstring, IResult,
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

            let open = chars[..halflen]
                .iter()
                .collect::<String>()
                .trim()
                .to_owned();
            let close = chars[halflen..]
                .iter()
                .collect::<String>()
                .trim()
                .to_owned();

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
            tuple((node_name(mode.clone()), opt(bracketed(mode.clone())))),
            map_node(mode.clone()),
        )(i)
    }
}

pub fn node_name(mode: ParseMode) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let max = i.len();
        let index = [
            i.find_substring(&mode.delimiter).unwrap_or(max),
            i.find_substring(&mode.brackets.open).unwrap_or(max),
            i.find_substring(&mode.brackets.close).unwrap_or(max),
        ]
        .into_iter()
        .min()
        .unwrap();

        if index > 0 {
            Ok((&i[index..], &i[..index]))
        } else {
            Err(nom::Err::Error(nom::error::Error {
                input: i,
                code: ErrorKind::Fail,
            }))
        }
    }
}

fn bracketed(mode: ParseMode) -> impl Fn(&str) -> IResult<&str, Vec<Node>> {
    move |i: &str| {
        delimited(
            wtag(mode.brackets.open.clone()),
            node_list(mode.clone()),
            wtag(mode.brackets.close.clone()),
        )(i)
    }
}

fn node_list(mode: ParseMode) -> impl Fn(&str) -> IResult<&str, Vec<Node>> {
    move |i: &str| {
        separated_list0(
            // FMT: -
            wtag(mode.delimiter.clone()),
            single_node(mode.clone()),
        )(i)
    }
}

fn wtag(wtag: String) -> impl Fn(&str) -> IResult<&str, (Option<&str>, &str)> {
    move |i: &str| {
        tuple((
            opt(take_while(|c: char| c.is_whitespace())),
            tag(wtag.as_str()),
        ))(i)
    }
}

fn map_node(mode: ParseMode) -> impl Fn((&str, Option<Vec<Node>>)) -> Result<Node, Infallible> {
    move |(name, children)| {
        Ok(Node::new(
            match mode.trim {
                true => name.trim().to_owned(),
                false => name.to_owned(),
            },
            children.unwrap_or_default(),
        ))
    }
}
