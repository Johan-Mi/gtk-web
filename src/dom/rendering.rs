use super::{Document, Element, Handle};
use crate::Browser;
use gtk::{
    glib::{clone, Propagation},
    pango::{AttrList, AttrSize},
    prelude::{ContainerExt, LabelExt, LinkButtonExt},
    Align, Box, Frame, Label, LinkButton, Orientation, Widget,
};
use html5ever::{local_name, tree_builder::NodeOrText};
use std::{
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

static FRAME: AtomicBool = AtomicBool::new(false);

impl Document {
    pub fn render(&self, browser: &Rc<Browser>) -> Widget {
        FRAME.store(std::env::var_os("FRAME").is_some(), Ordering::Relaxed);
        self.elements[&Handle(0)]
            .render(self, browser)
            .unwrap_or_else(|| Box::default().into())
    }
}

impl Element {
    pub fn render(
        &self,
        document: &Document,
        browser: &Rc<Browser>,
    ) -> Option<Widget> {
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
                        .and_then(|it| it.render(document, browser))
                    {
                        contains_something = true;
                        widget.add(&node);
                    }
                }
                NodeOrText::AppendText(text) => {
                    contains_something = true;
                    if let (Some(href), &local_name!("a")) = (
                        self.attrs.iter().find_map(|it| {
                            (it.name.local == local_name!("href"))
                                .then_some(&*it.value)
                        }),
                        &self.name.local,
                    ) {
                        let link = LinkButton::with_label(href, text);
                        link.connect_activate_link(
                            clone!(@strong browser => move |link| {
                                link.uri().map_or(Propagation::Proceed, |uri| {
                                    browser.open(&uri);
                                    Propagation::Stop
                                })
                            }),
                        );
                        widget.add(&link);
                    } else {
                        let label = label(text);
                        self.style_label(&label);
                        widget.add(&label);
                    }
                }
            }
        }
        contains_something.then(|| {
            if FRAME.load(Ordering::Relaxed) {
                Frame::builder()
                    .label(&*self.name.local)
                    .child(&widget)
                    .build()
                    .into()
            } else {
                widget.into()
            }
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
    Label::builder()
        .label(text)
        .halign(Align::Start)
        .wrap(true)
        .build()
}
