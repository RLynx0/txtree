use std::char;

use clap::builder::{ValueParser, ValueParserFactory};

#[derive(Debug, Clone)]
pub struct Symbols {
    pub(super) vertical: char,
    pub(super) horizontal: char,
    pub(super) crossing: char,
    pub(super) branch_right: char,
    pub(super) branch_left: char,
    pub(super) branch_down: char,
    pub(super) branch_up: char,
    pub(super) corner_down_right: char,
    pub(super) corner_down_left: char,
    pub(super) corner_up_right: char,
    pub(super) corner_up_left: char,
    pub(super) cap_right: String,
    pub(super) cap_left: String,
}
impl Symbols {
    pub fn rounded() -> Self {
        Symbols {
            vertical: '│',
            horizontal: '─',
            crossing: '┼',
            branch_right: '├',
            branch_left: '┤',
            branch_down: '┬',
            branch_up: '┴',
            corner_down_right: '╭',
            corner_down_left: '╮',
            corner_up_right: '╰',
            corner_up_left: '╯',
            cap_right: String::from("╴"),
            cap_left: String::from("╶"),
        }
    }
    pub fn ascii() -> Self {
        Symbols {
            vertical: '|',
            horizontal: '-',
            crossing: '+',
            branch_right: '+',
            branch_left: '+',
            branch_down: '+',
            branch_up: '+',
            corner_down_right: '+',
            corner_down_left: '+',
            corner_up_right: '+',
            corner_up_left: '+',
            cap_right: String::from("- "),
            cap_left: String::from(" -"),
        }
    }
}

impl Default for Symbols {
    fn default() -> Self {
        Symbols {
            vertical: '│',
            horizontal: '─',
            crossing: '┼',
            branch_right: '├',
            branch_left: '┤',
            branch_down: '┬',
            branch_up: '┴',
            corner_down_right: '┌',
            corner_down_left: '┐',
            corner_up_right: '└',
            corner_up_left: '┘',
            cap_right: String::from("╴"),
            cap_left: String::from("╶"),
        }
    }
}

impl ValueParserFactory for Symbols {
    type Parser = ValueParser;

    fn value_parser() -> Self::Parser {
        ValueParser::new(|string: &str| -> Result<Self, &'static str> {
            let mut chars = string.chars();
            macro_rules! next {
                () => {
                    chars.next().ok_or("Not enough characters in string")?
                };
            }
            let vertical = next!();
            let horizontal = next!();
            let crossing = next!();
            let branch_right = next!();
            let branch_left = next!();
            let branch_down = next!();
            let branch_up = next!();
            let corner_down_right = next!();
            let corner_down_left = next!();
            let corner_up_right = next!();
            let corner_up_left = next!();

            let remaining = chars.collect::<Vec<_>>();
            let halflen = remaining.len() / 2;
            let cap_right = remaining[..halflen].iter().collect();
            let cap_left = remaining[halflen..].iter().collect();

            Ok(Symbols {
                vertical,
                horizontal,
                crossing,
                branch_right,
                branch_left,
                branch_down,
                branch_up,
                corner_down_right,
                corner_down_left,
                corner_up_right,
                corner_up_left,
                cap_right,
                cap_left,
            })
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct SymbolsBuilder {
    default_to_ascii: bool,
    default_to_rounded: bool,

    vertical: Option<char>,
    horizontal: Option<char>,
    crossing: Option<char>,
    branch_right: Option<char>,
    branch_left: Option<char>,
    branch_down: Option<char>,
    branch_up: Option<char>,
    corner_down_right: Option<char>,
    corner_down_left: Option<char>,
    corner_up_right: Option<char>,
    corner_up_left: Option<char>,
    cap_right: Option<String>,
    cap_left: Option<String>,
}
impl SymbolsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default_to_ascii(&mut self, default_to_ascii: bool) -> &mut Self {
        self.default_to_ascii = default_to_ascii;
        self
    }
    pub fn default_to_rounded(&mut self, default_to_rounded: bool) -> &mut Self {
        self.default_to_rounded = default_to_rounded;
        self
    }
    pub fn opt_symbol_set(&mut self, symbols: Option<Symbols>) -> &mut Self {
        if let Some(symbols) = symbols {
            self.vertical = Some(symbols.vertical);
            self.horizontal = Some(symbols.horizontal);
            self.cap_right = Some(symbols.cap_right);
            self.cap_left = Some(symbols.cap_left);
            self.crossing = Some(symbols.crossing);
            self.branch_right = Some(symbols.branch_right);
            self.branch_left = Some(symbols.branch_left);
            self.branch_down = Some(symbols.branch_down);
            self.branch_up = Some(symbols.branch_up);
            self.corner_down_right = Some(symbols.corner_down_right);
            self.corner_down_left = Some(symbols.corner_down_left);
            self.corner_up_right = Some(symbols.corner_up_right);
            self.corner_up_left = Some(symbols.corner_up_left);
        }
        self
    }

    pub fn vertical(&mut self, vertical: Option<char>) -> &mut Self {
        self.vertical = vertical;
        self
    }
    pub fn horizontal(&mut self, horizontal: Option<char>) -> &mut Self {
        self.horizontal = horizontal;
        self
    }
    pub fn crossing(&mut self, crossing: Option<char>) -> &mut Self {
        self.crossing = crossing;
        self
    }
    pub fn branch_right(&mut self, branch_right: Option<char>) -> &mut Self {
        self.branch_right = branch_right;
        self
    }
    pub fn branch_left(&mut self, branch_left: Option<char>) -> &mut Self {
        self.branch_left = branch_left;
        self
    }
    pub fn branch_down(&mut self, branch_down: Option<char>) -> &mut Self {
        self.branch_down = branch_down;
        self
    }
    pub fn branch_up(&mut self, branch_up: Option<char>) -> &mut Self {
        self.branch_up = branch_up;
        self
    }
    pub fn corner_down_right(&mut self, corner_down_right: Option<char>) -> &mut Self {
        self.corner_down_right = corner_down_right;
        self
    }
    pub fn corner_down_left(&mut self, corner_down_left: Option<char>) -> &mut Self {
        self.corner_down_left = corner_down_left;
        self
    }
    pub fn corner_up_right(&mut self, corner_up_right: Option<char>) -> &mut Self {
        self.corner_up_right = corner_up_right;
        self
    }
    pub fn corner_up_left(&mut self, corner_up_left: Option<char>) -> &mut Self {
        self.corner_up_left = corner_up_left;
        self
    }
    pub fn cap_right(&mut self, cap_right: Option<String>) -> &mut Self {
        self.cap_right = cap_right;
        self
    }
    pub fn cap_left(&mut self, cap_left: Option<String>) -> &mut Self {
        self.cap_left = cap_left;
        self
    }

    pub fn build(&self) -> Symbols {
        macro_rules! field {
            ($field: tt) => {
                match (&self.$field, self.default_to_ascii, self.default_to_rounded) {
                    (Some(c), _, _) => c.to_owned(),
                    (None, true, _) => Symbols::ascii().$field,
                    (None, false, true) => Symbols::rounded().$field,
                    (None, false, false) => Symbols::default().$field,
                }
            };
        }

        Symbols {
            vertical: field!(vertical),
            horizontal: field!(horizontal),
            crossing: field!(crossing),
            branch_right: field!(branch_right),
            branch_left: field!(branch_left),
            branch_down: field!(branch_down),
            branch_up: field!(branch_up),
            corner_down_right: field!(corner_down_right),
            corner_down_left: field!(corner_down_left),
            corner_up_right: field!(corner_up_right),
            corner_up_left: field!(corner_up_left),
            cap_right: field!(cap_right),
            cap_left: field!(cap_left),
        }
    }
}
