use std::{
    io::{self, BufRead},
    process::exit,
};

use clap::Parser;
use txtree::{parser::parse_nodes, prelude::*};

use crate::opt::Opt;

mod opt;

macro_rules! print_debug {
    ($c: expr, $($p: expr),*) => {
        if $c {
            $(eprintln!("{:#?}", $p);)*
        }
    };
}

fn main() {
    let opt = Opt::parse();

    let render_mode = RenderMode::new(
        SymbolsBuilder::new()
            .opt_symbol_set(opt.symbols)
            .default_to_ascii(opt.ascii)
            .default_to_rounded(opt.rounded)
            .build(),
        opt.lines,
        opt.gaps,
        opt.chain,
        opt.vertical,
        opt.invert,
        AlignmentBuilder::new()
            .flipped(opt.flip)
            .centered(opt.middle)
            .build(),
    );
    print_debug!(opt.debug, render_mode);

    let parse_mode = ParseMode::new(opt.delimiter, opt.brackets);
    print_debug!(opt.debug, parse_mode);

    if opt.input.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.expect("Failed to read from stdin");
            match parse_and_render(&line, parse_mode.clone(), opt.debug) {
                Ok(s) => println!("{}", s),
                Err(e) => eprintln!("{}", e),
            };
        }
    } else {
        match parse_and_render(&opt.input.join(" "), parse_mode, opt.debug) {
            Ok(s) => println!("{}", s),
            Err(e) => {
                eprintln!("{}", e);
                exit(1)
            }
        }
    }
}

fn parse_and_render(input: &str, parse_mode: ParseMode, debug: bool) -> Result<String, String> {
    let parsed = match parse_nodes(input, parse_mode) {
        Ok(("", p)) => p,
        Ok((s, _)) => return Err(format!("Failed to parse input, on {:?}.", s)),
        Err(e) => return Err(format!("Failed to parse input. {:?}", e)),
    };
    print_debug!(debug, parsed);

    Ok(String::new())
}
