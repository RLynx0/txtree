use clap::Parser;
use txtree::{parser::Brackets, render::Symbols};

/// Utility to generate text-based tree graphs
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Opt {
    /// Strings to be turned into a graph
    pub(crate) input: Vec<String>,

    /// String used to separate elements
    #[arg(short, long, default_value = ",")]
    pub(crate) delimiter: String,

    /// Strings used to enclose child elements
    ///
    /// Requires a string of at least 2 characters.
    /// If more are provided, the input is split evenly.
    ///
    /// Surrounding whitespace is trimmed after splitting!
    /// This allows for strings of differing length:
    /// --brackets "BEGIN  END"
    /// => ("BEGIN", "  END")   [split]
    /// => ("BEGIN", "END")     [trim]
    ///
    /// Try --debug if the parser isn't behaving as intended!
    #[arg(short, long, default_value = "[]", verbatim_doc_comment)]
    pub(crate) brackets: Brackets,

    /// Trim whitespace from element names
    #[arg(short, long)]
    pub(crate) trim: bool,

    /// Number of lines between elements
    #[arg(short, long, default_value_t = 0)]
    pub(crate) lines: u8,

    /// Number of columns between elements
    #[arg(short, long, default_value_t = 0)]
    pub(crate) gaps: u8,

    /// Use horizontal branches
    #[arg(short, long)]
    pub(crate) chain: bool,

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
    /// Provide a symbol set to use instead of "│─┼├┤┬┴┌┐└┘╴╶".
    ///
    /// Requires a string of at least 11 characters.
    /// These are used for:
    ///   1. Vertical lines
    ///   2. Horizontal lines
    ///   3. Crossings
    ///   4. Vertical line with branch right
    ///   5. Vertical line with branch left
    ///   6. Horizontal line with branch down
    ///   7. Horizontal line with branch up
    ///   8. Corner connecting down and right
    ///   9. Corner connecting down and left
    ///   10. Corner connecting up and right
    ///   11. Corner connecting up and left
    ///
    /// Remaining characters are split evenly for right and left branch caps.
    ///
    /// Try --debug if the renderer isn't bahaving as intended!
    #[arg(short, long, verbatim_doc_comment)]
    pub(crate) symbols: Option<Symbols>,

    /// Sort child elements alphabetically
    ///
    /// Does nothing when using --sort-by
    #[arg(short = 'S', long)]
    pub(crate) sort: bool,

    /// Sort child elements with external program
    #[arg(short = 'B', long, name = "PROGRAM")]
    pub(crate) sort_by: Option<String>,

    /// Reverse order of child elements
    #[arg(short = 'R', long)]
    pub(crate) reverse: bool,

    /// Print debug information
    #[arg(short = 'D', long)]
    pub(crate) debug: bool,
}
