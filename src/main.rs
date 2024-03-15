use std::{
    io::{self, BufRead},
    process::exit,
};

use clap::Parser;
use txtree::{prelude::*, OrderMode};

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

    let parse_mode = ParseMode::new(opt.delimiter, opt.brackets, opt.trim);

    let order_mode = OrderModeBuilder::new()
        .opt_sort_program(opt.sort_by)
        .default_to_alphabetical(opt.sort)
        .reverse(opt.reverse)
        .build();

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

    print_debug!(opt.debug, parse_mode);
    print_debug!(opt.debug, order_mode);
    print_debug!(opt.debug, render_mode);

    if opt.input.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.expect("Failed to read from stdin");
            match render_string(&line, parse_mode.clone(), &order_mode, opt.debug) {
                Ok(s) => println!("{}", s),
                Err(e) => eprintln!("{}", e),
            };
        }
    } else {
        match render_string(&opt.input.join(" "), parse_mode, &order_mode, opt.debug) {
            Ok(s) => println!("{}", s),
            Err(e) => {
                eprintln!("{}", e);
                exit(1)
            }
        }
    }
}

fn render_string(
    input: &str,
    parse_mode: ParseMode,
    order_mode: &OrderMode,
    debug: bool,
) -> Result<String, String> {
    let mut parsed = Node::new(
        String::from("ROOT"),
        match parse_nodes(input, parse_mode) {
            Ok(("", p)) => p,
            Ok((s, _)) => return Err(format!("Failed to parse input, on {:?}.", s)),
            Err(e) => return Err(format!("Failed to parse input. {:?}", e)),
        },
    );

    print_debug!(debug, parsed);
    parsed.order_children(order_mode);
    print_debug!(debug, parsed);

    Ok(parsed
        .take_children()
        .into_iter()
        .map(|n| format!("{}\n\n", n.mock_display()))
        .collect())
}
