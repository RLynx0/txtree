use clap::Parser;
use txtree::{parser::Brackets, render::Symbols};

/// Utility to generate a text-based tree graph
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Opt {
    /// Strings to be turned into a graph
    pub(crate) input: Vec<String>,

    /// Sort child elements alphabetically
    #[arg(short, long)]
    pub(crate) sort: bool,

    /// Character used to separate elements
    #[arg(short, long, default_value_t = ',')]
    pub(crate) delimiter: char,

    /// Characters used to enclose child elements
    #[arg(short, long, default_value = "[]")]
    pub(crate) brackets: Brackets,

    /// Number of lines between elements
    #[arg(short, long, default_value_t = 0)]
    pub(crate) lines: u8,

    /// Number of columns between elements
    #[arg(short, long, default_value_t = 0)]
    pub(crate) gaps: u8,

    /// Take an extra step before branching
    #[arg(short, long)]
    pub(crate) extend: bool,

    /// Grow tree vertically
    #[arg(short, long)]
    pub(crate) vertical: bool,

    /// Grow tree in the opposite direction
    #[arg(short, long)]
    pub(crate) invert: bool,

    /// Grow branches in the opposite direction
    ///
    /// Does nothing when using --middle.
    #[arg(short, long)]
    pub(crate) flip: bool,

    /// Branch out on both sides
    #[arg(short, long)]
    pub(crate) middle: bool,

    /// Use rounded corners
    ///
    /// Does nothing when using --ascii or --symbols.
    #[arg(short, long)]
    pub(crate) rounded: bool,

    /// Use ascii symbol set
    ///
    /// Does nothing when using --symbols.
    #[arg(short, long)]
    pub(crate) ascii: bool,

    /// Use custom symbol set
    ///
    /// Provide a symbol set to use instead
    /// of "│─╴╶┼├┤┬┴┌┐└┘" (13 characters).
    #[arg(short = 'S', long)]
    pub(crate) symbols: Option<Symbols>,

    /// Print debug information
    #[arg(short = 'D', long)]
    pub(crate) debug: bool,
}
