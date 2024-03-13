pub mod parser;
pub mod render;

pub mod prelude {
    pub use super::{
        parser::{parse_nodes, Brackets, ParseMode},
        render::{AlignmentBuilder, RenderMode, SymbolsBuilder},
        Node,
    };
}

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    children: Vec<Node>,
}
impl Node {
    pub fn new(name: String, children: Vec<Node>) -> Self {
        Node { name, children }
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
