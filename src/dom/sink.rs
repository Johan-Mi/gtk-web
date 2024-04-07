use super::{Document, Element, Handle};
use html5ever::{
    interface::{ElementFlags, NodeOrText, QuirksMode, TreeSink},
    local_name, namespace_url, ns,
    tendril::StrTendril,
    Attribute, ExpandedName, QualName,
};
use std::{borrow::Cow, collections::HashMap};

pub struct Sink {
    pub document: Document,
    next_id: usize,
}

impl Sink {
    pub fn new() -> Self {
        Self {
            document: Document {
                elements: HashMap::from([(
                    Handle(0),
                    Element {
                        name: QualName {
                            prefix: None,
                            ns: ns!(html),
                            local: local_name!("root"),
                        },
                        children: Vec::new(),
                    },
                )]),
            },
            next_id: 1,
        }
    }

    fn new_handle(&mut self) -> Handle {
        let id = self.next_id;
        self.next_id += 1;
        Handle(id)
    }
}

impl TreeSink for Sink {
    type Handle = Handle;
    type Output = Document;

    fn finish(self) -> Document {
        self.document
    }

    fn get_document(&mut self) -> Handle {
        Handle(0)
    }

    fn get_template_contents(&mut self, _: &Handle) -> Handle {
        Handle::INVALID
    }

    fn same_node(&self, x: &Handle, y: &Handle) -> bool {
        x == y
    }

    fn elem_name(&self, target: &Handle) -> ExpandedName {
        self.document.elements[target].name.expanded()
    }

    fn create_element(
        &mut self,
        name: QualName,
        _: Vec<Attribute>,
        _: ElementFlags,
    ) -> Handle {
        let handle = self.new_handle();
        let element = Element {
            name,
            children: Vec::new(),
        };
        self.document.elements.insert(handle, element);
        handle
    }

    fn create_comment(&mut self, _text: StrTendril) -> Handle {
        Handle::INVALID
    }

    fn create_pi(&mut self, _: StrTendril, _: StrTendril) -> Handle {
        Handle::INVALID
    }

    fn append_before_sibling(&mut self, _: &Handle, _: NodeOrText<Handle>) {}

    fn append_based_on_parent_node(
        &mut self,
        _: &Handle,
        _: &Handle,
        _: NodeOrText<Handle>,
    ) {
    }

    fn parse_error(&mut self, _: Cow<'static, str>) {}

    fn set_quirks_mode(&mut self, _: QuirksMode) {}

    fn append(&mut self, parent: &Handle, child: NodeOrText<Handle>) {
        self.document
            .elements
            .get_mut(parent)
            .unwrap()
            .children
            .push(child);
    }

    fn append_doctype_to_document(
        &mut self,
        _: StrTendril,
        _: StrTendril,
        _: StrTendril,
    ) {
    }

    fn add_attrs_if_missing(&mut self, _: &Handle, _: Vec<Attribute>) {}

    fn remove_from_parent(&mut self, _target: &Handle) {}

    fn reparent_children(&mut self, _node: &Handle, _new_parent: &Handle) {}

    fn mark_script_already_started(&mut self, _node: &Handle) {}
}
