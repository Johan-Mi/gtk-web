mod sink;
pub use sink::Sink;

use gtk::{
    pango::{AttrList, AttrSize},
    prelude::{ContainerExt, LabelExt},
    Align, Frame, Label, Orientation, Widget,
};
use html5ever::{local_name, tree_builder::NodeOrText, QualName};
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
    pub fn render(&self) -> Widget {
        self.elements[&Handle(0)]
            .render(self)
            .unwrap_or_else(|| gtk::Box::default().into())
    }
}

impl Element {
    pub fn render(&self, document: &Document) -> Option<Widget> {
        if self.is_invisible() {
            return None;
        }

        let mut contains_something = false;
        let widget = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .halign(Align::Start)
            .build();
        for child in &self.children {
            match child {
                NodeOrText::AppendNode(node) => {
                    if let Some(node) = document
                        .elements
                        .get(node)
                        .and_then(|it| it.render(document))
                    {
                        contains_something = true;
                        widget.add(&node);
                    }
                }
                NodeOrText::AppendText(text) => {
                    contains_something = true;
                    let label = label(text);
                    if self.name.local == local_name!("h1") {
                        let attrs = AttrList::new();
                        attrs.insert(AttrSize::new(32 * gtk::pango::SCALE));
                        label.set_attributes(Some(&attrs));
                    }
                    widget.add(&label);
                }
            }
        }
        contains_something.then(|| {
            Frame::builder()
                .label(&*self.name.local)
                .child(&widget)
                .build()
                .into()
        })
    }

    fn is_invisible(&self) -> bool {
        self.name.local == local_name!("head")
    }
}

fn label(text: &str) -> Label {
    Label::builder().label(text).halign(Align::Start).build()
}
