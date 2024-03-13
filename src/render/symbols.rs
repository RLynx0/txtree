use clap::builder::{ValueParser, ValueParserFactory};

#[derive(Debug, Clone)]
pub struct Symbols {
    pub(super) vertical: char,
    pub(super) horizontal: char,
    pub(super) cap_right: char,
    pub(super) cap_left: char,
    pub(super) crossing: char,
    pub(super) branch_right: char,
    pub(super) branch_left: char,
    pub(super) branch_down: char,
    pub(super) branch_up: char,
    pub(super) corner_down_right: char,
    pub(super) corner_down_left: char,
    pub(super) corner_up_right: char,
    pub(super) corner_up_left: char,
}
impl Symbols {
    pub fn rounded() -> Self {
        Self::ROUNDED
    }
    pub fn ascii() -> Self {
        Self::ASCII
    }

    pub(crate) const DEFAULT: Self = Symbols {
        vertical: '│',
        horizontal: '─',
        cap_right: '╴',
        cap_left: '╶',
        crossing: '┼',
        branch_right: '├',
        branch_left: '┤',
        branch_down: '┬',
        branch_up: '┴',
        corner_down_right: '┌',
        corner_down_left: '┐',
        corner_up_right: '└',
        corner_up_left: '┘',
    };
    pub(crate) const ROUNDED: Self = Symbols {
        vertical: '│',
        horizontal: '─',
        cap_right: '╴',
        cap_left: '╶',
        crossing: '┼',
        branch_right: '├',
        branch_left: '┤',
        branch_down: '┬',
        branch_up: '┴',
        corner_down_right: '╭',
        corner_down_left: '╮',
        corner_up_right: '╰',
        corner_up_left: '╯',
    };
    pub(crate) const ASCII: Self = Symbols {
        vertical: '|',
        horizontal: '-',
        cap_right: ' ',
        cap_left: ' ',
        crossing: '+',
        branch_right: '+',
        branch_left: '+',
        branch_down: '+',
        branch_up: '+',
        corner_down_right: '+',
        corner_down_left: '+',
        corner_up_right: '+',
        corner_up_left: '+',
    };
}

impl Default for Symbols {
    fn default() -> Self {
        Self::DEFAULT
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

            let symbols = Symbols {
                vertical: next!(),
                horizontal: next!(),
                cap_right: next!(),
                cap_left: next!(),
                crossing: next!(),
                branch_right: next!(),
                branch_left: next!(),
                branch_down: next!(),
                branch_up: next!(),
                corner_down_right: next!(),
                corner_down_left: next!(),
                corner_up_right: next!(),
                corner_up_left: next!(),
            };

            match chars.next() {
                None => Ok(symbols),
                Some(_) => Err("Too many characters in string"),
            }
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct SymbolsBuilder {
    default_to_ascii: bool,
    default_to_rounded: bool,

    vertical: Option<char>,
    horizontal: Option<char>,
    cap_right: Option<char>,
    cap_left: Option<char>,
    crossing: Option<char>,
    branch_right: Option<char>,
    branch_left: Option<char>,
    branch_down: Option<char>,
    branch_up: Option<char>,
    corner_down_right: Option<char>,
    corner_down_left: Option<char>,
    corner_up_right: Option<char>,
    corner_up_left: Option<char>,
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
    pub fn cap_right(&mut self, cap_right: Option<char>) -> &mut Self {
        self.cap_right = cap_right;
        self
    }
    pub fn cap_left(&mut self, cap_left: Option<char>) -> &mut Self {
        self.cap_left = cap_left;
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

    pub fn build(&self) -> Symbols {
        macro_rules! field {
            ($field: tt) => {
                match (self.$field, self.default_to_ascii, self.default_to_rounded) {
                    (Some(c), _, _) => c,
                    (None, true, _) => Symbols::ascii().$field,
                    (None, false, true) => Symbols::rounded().$field,
                    (None, false, false) => Symbols::default().$field,
                }
            };
        }

        Symbols {
            vertical: field!(vertical),
            horizontal: field!(horizontal),
            cap_right: field!(cap_right),
            cap_left: field!(cap_left),
            crossing: field!(crossing),
            branch_right: field!(branch_right),
            branch_left: field!(branch_left),
            branch_down: field!(branch_down),
            branch_up: field!(branch_up),
            corner_down_right: field!(corner_down_right),
            corner_down_left: field!(corner_down_left),
            corner_up_right: field!(corner_up_right),
            corner_up_left: field!(corner_up_left),
        }
    }
}
