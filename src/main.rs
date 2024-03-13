use std::process::exit;

use clap::Parser;
use txtree::{parser::parse_nodes, prelude::*};

use crate::opt::Opt;

mod opt;

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
        opt.extend,
        opt.vertical,
        opt.invert,
        AlignmentBuilder::new()
            .flipped(opt.flip)
            .centered(opt.middle)
            .build(),
    );
    if opt.debug {
        eprintln!("{:#?}\n", render_mode);
        eprintln!("Demo Grid {{\n{}\n}}\n", render_mode.demo_grid());
    }

    let parse_mode = ParseMode::new(opt.delimiter, opt.brackets);
    if opt.debug {
        eprintln!("{:#?}\n", parse_mode);
    }

    let parsed = match parse_nodes(&opt.input.join(" "), parse_mode) {
        Ok(("", p)) => p,
        Ok((s, _)) => {
            eprintln!("Failed to parse input, on {:?}.", s);
            exit(1);
        }
        Err(e) => {
            eprintln!("Failed to parse input. {:?}", e);
            exit(1);
        }
    };
    if opt.debug {
        eprintln!("Parsed Nodes {:#?}\n", parsed);
    }
}
