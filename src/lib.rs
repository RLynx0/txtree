pub mod parser;
pub mod render;

pub mod prelude {
    pub use super::{
        parser::{parse_nodes, Brackets, ParseMode},
        render::{AlignmentBuilder, RenderMode, SymbolsBuilder},
        Node, OrderModeBuilder,
    };
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    name: String,
    children: Vec<Node>,
}
impl Node {
    pub fn new(name: String, children: Vec<Node>) -> Self {
        Node { name, children }
    }
    pub fn order_children(&mut self, order_mode: &OrderMode) {
        match order_mode.sort.as_ref() {
            Some(Sorter::Alphabetical) => self.children.sort(),
            Some(Sorter::External(prog)) => todo!("Sort using {}", prog),
            None => (),
        }

        if order_mode.reverse {
            self.children.reverse();
        }

        self.children
            .iter_mut()
            .for_each(|c| c.order_children(order_mode));
    }
    pub fn take_children(self) -> Vec<Node> {
        self.children
    }
}
impl<S, I> From<(S, I)> for Node
where
    String: From<S>,
    Vec<Node>: From<I>,
{
    fn from((name, children): (S, I)) -> Self {
        Node::new(name.into(), children.into())
    }
}

#[derive(Debug, Clone)]
pub struct OrderMode {
    sort: Option<Sorter>,
    reverse: bool,
}
#[derive(Debug, Clone)]
pub enum Sorter {
    Alphabetical,
    External(String),
}

#[derive(Debug, Clone, Default)]
pub struct OrderModeBuilder {
    sort_program: Option<String>,
    default_to_alphabetical: bool,
    reverse_order: bool,
}
impl OrderModeBuilder {
    pub fn new() -> Self {
        OrderModeBuilder::default()
    }

    pub fn default_to_alphabetical(&mut self, value: bool) -> &mut Self {
        self.default_to_alphabetical = value;
        self
    }
    pub fn opt_sort_program(&mut self, program: Option<String>) -> &mut Self {
        self.sort_program = program;
        self
    }
    pub fn reverse(&mut self, reverse: bool) -> &mut Self {
        self.reverse_order = reverse;
        self
    }

    pub fn build(&self) -> OrderMode {
        OrderMode {
            sort: self
                .sort_program
                .as_ref()
                .map(|p| Sorter::External(p.clone()))
                .or(match self.default_to_alphabetical {
                    true => Some(Sorter::Alphabetical),
                    false => None,
                }),
            reverse: self.reverse_order,
        }
    }
}

enum DisplayPlace {
    Item,
    Last,
}

const PRE_LINE: &str = "│ ";
const PRE_ITEM: &str = "├╴";
const PRE_LAST: &str = "╰╴";
const PRE_EMPT: &str = "  ";

impl Node {
    pub fn mock_display(&self) -> String {
        self.display_branch(0, 0, "")
    }

    fn display_root(&self, preprint: &str) -> String {
        let root_length = self.children.len();
        self.children
            .iter()
            .enumerate()
            .map(|(index, child)| {
                child.display_branch(
                    // FMT: -
                    root_length,
                    index,
                    preprint,
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn display_branch(&self, root_len: usize, index: usize, preprint: &str) -> String {
        let len = self.children.len();
        let placement = compute_placement(root_len, index);
        let child_preprint = compute_child_preprint(preprint, &placement);
        let preprint = compute_preprint(preprint, &placement);
        let name_separator = compute_name_separator(len);

        format!(
            "{}{}{}{}",
            preprint,
            self.name,
            name_separator,
            self.display_root(&child_preprint)
        )
    }
}

fn compute_placement(root_len: usize, index: usize) -> DisplayPlace {
    let last_index = root_len.saturating_sub(1);
    if index == last_index {
        DisplayPlace::Last
    } else {
        DisplayPlace::Item
    }
}

fn compute_child_preprint(preprint: &str, placement: &DisplayPlace) -> String {
    use DisplayPlace::*;
    match placement {
        Item => format!("{}{}", preprint, PRE_LINE),
        Last => format!("{}{}", preprint, PRE_EMPT),
    }
}

fn compute_preprint(preprint: &str, placement: &DisplayPlace) -> String {
    use DisplayPlace::*;
    match placement {
        Item => format!("{}{}", preprint, PRE_ITEM),
        Last => format!("{}{}", preprint, PRE_LAST),
    }
}

fn compute_name_separator(len: usize) -> &'static str {
    match len {
        0 => "",
        _ => "\n",
    }
}
