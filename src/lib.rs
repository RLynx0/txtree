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
        self.children.sort();

        if order_mode.reverse {
            self.children.reverse();
        }

        self.children
            .iter_mut()
            .for_each(|c| c.order_children(order_mode));
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
