use super::{Document, Element, Handle};
use gtk::{
    pango::{AttrList, AttrSize},
    prelude::{ContainerExt, LabelExt},
    Align, Box, Frame, Label, Orientation, Widget,
};
use html5ever::{local_name, tree_builder::NodeOrText};

impl Document {
    pub fn render(&self) -> Widget {
        self.elements[&Handle(0)]
            .render(self)
            .unwrap_or_else(|| Box::default().into())
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
                    self.style_label(&label);
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

    fn style_label(&self, label: &Label) {
        if let Some(scale) = match self.name.local {
            local_name!("h1") => Some(32),
            local_name!("h2") => Some(24),
            local_name!("h3") => Some(19),
            local_name!("h4") => Some(16),
            local_name!("h5") => Some(13),
            local_name!("h6") => Some(11),
            _ => None,
        } {
            let attrs = AttrList::new();
            attrs.insert(AttrSize::new(scale * gtk::pango::SCALE));
            label.set_attributes(Some(&attrs));
        }
    }
}

fn label(text: &str) -> Label {
    Label::builder().label(text).halign(Align::Start).build()
}
