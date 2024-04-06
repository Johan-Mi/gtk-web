use html5ever::{
    interface::{ElementFlags, NodeOrText, QuirksMode, TreeSink},
    tendril::StrTendril,
    Attribute, ExpandedName, QualName,
};
use std::{borrow::Cow, collections::HashMap};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(usize);

#[derive(Default)]
pub struct Sink {
    next_id: usize,
    pub names: HashMap<Handle, QualName>,
}

impl Sink {
    fn new_handle(&mut self) -> Handle {
        let id = self.next_id;
        self.next_id += 1;
        Handle(id)
    }
}

impl TreeSink for Sink {
    type Handle = Handle;
    type Output = Self;
    fn finish(self) -> Self {
        self
    }

    fn get_document(&mut self) -> Handle {
        Handle(0)
    }

    fn get_template_contents(&mut self, _: &Handle) -> Handle {
        unimplemented!()
    }

    fn same_node(&self, x: &Handle, y: &Handle) -> bool {
        x == y
    }

    fn elem_name(&self, target: &Handle) -> ExpandedName {
        self.names[target].expanded()
    }

    fn create_element(
        &mut self,
        name: QualName,
        _: Vec<Attribute>,
        _: ElementFlags,
    ) -> Handle {
        let handle = self.new_handle();
        self.names.insert(handle, name);
        handle
    }

    fn create_comment(&mut self, _text: StrTendril) -> Handle {
        self.new_handle()
    }

    fn create_pi(&mut self, _: StrTendril, _: StrTendril) -> Handle {
        unimplemented!()
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

    fn append(&mut self, _: &Handle, _: NodeOrText<Handle>) {}

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
