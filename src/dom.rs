mod rendering;
mod sink;
pub use sink::Sink;

use html5ever::{tree_builder::NodeOrText, QualName};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(usize);

impl Handle {
    const INVALID: Self = Self(usize::MAX);
}

pub struct Document {
    elements: HashMap<Handle, Element>,
}

struct Element {
    name: QualName,
    children: Vec<NodeOrText<Handle>>,
}
