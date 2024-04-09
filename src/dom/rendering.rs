use super::{Document, Element, Handle};
use crate::Browser;
use gtk::{
    glib::{clone, markup_escape_text, Propagation},
    pango::{AttrList, AttrSize},
    prelude::{ContainerExt, LabelExt, LinkButtonExt},
    Align, Box, Frame, Label, LinkButton, Orientation, Widget,
};
use html5ever::{local_name, tree_builder::NodeOrText};
use std::rc::Rc;

impl Document {
    pub fn render(&self, browser: &Rc<Browser>) -> Widget {
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
        let orientation = if self.name.local == local_name!("p") {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        };
        let widget = gtk::Box::builder()
            .orientation(orientation)
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
                                    browser.open(&uri, false);
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
            if browser.frame {
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

    const fn is_invisible(&self) -> bool {
        matches!(
            self.name.local,
            local_name!("head") | local_name!("script") | local_name!("style")
        )
    }

    fn style_label(&self, label: &Label) {
        if matches!(self.name.local, local_name!("i") | local_name!("em")) {
            label.set_markup(&format!(
                "<i>{}</i>",
                markup_escape_text(&label.text())
            ));
        } else if matches!(
            self.name.local,
            local_name!("b") | local_name!("strong")
        ) {
            label.set_markup(&format!(
                "<b>{}</b>",
                markup_escape_text(&label.text())
            ));
        } else if let Some(scale) = match self.name.local {
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
