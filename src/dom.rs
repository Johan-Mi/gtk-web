mod sink;
pub use sink::Sink;

use gtk::{glib::IsA, Align, Frame, Label, Orientation, Widget};
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

impl Document {
    pub fn render(&self) -> impl IsA<Widget> {
        self.elements[&Handle(0)].render(self)
    }
}

impl Element {
    pub fn render(&self, document: &Document) -> impl IsA<Widget> {
        let mut widget = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .halign(Align::Start);
        for child in &self.children {
            widget = match child {
                NodeOrText::AppendNode(node) => {
                    if let Some(node) = document.elements.get(node) {
                        widget.child(&node.render(document))
                    } else {
                        widget
                    }
                }
                NodeOrText::AppendText(text) => widget.child(&label(text)),
            }
        }
        Frame::builder()
            .label(&*self.name.local)
            .child(&widget.build())
            .build()
    }
}

fn label(text: &str) -> Label {
    Label::builder().label(text).halign(Align::Start).build()
}
