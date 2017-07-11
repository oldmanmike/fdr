use std::io;

use remote_debugger::css::InlineTextBox;
use remote_debugger::dom::BackendNodeId;
use remote_debugger::dom::PseudoType;
use remote_debugger::dom::Rect;
use remote_debugger::page::FrameId;

struct DOMNode {
    node_type: i32,
    node_name: String,
    node_value: String,
    backend_node_id: BackendNodeId,
    child_node_indexes: Option<Vec<i32>>,
    attributes: Option<Vec<NameValue>>,
    pseudo_element_indexes: Option<Vec<i32>>,
    layout_node_index: Option<i32>,
    document_url: Option<String>,
    base_url: Option<String>,
    content_language: Option<String>,
    public_id: Option<String>,
    system_id: Option<String>,
    frame_id: Option<FrameId>,
    content_document_index: Option<i32>,
    imported_document_index: Option<i32>,
    template_content_index: Option<i32>,
    pseudo_type: Option<PseudoType>,
    is_clickable: Option<bool>,
}

struct LayoutTreeNode {
    dom_node_index: i32,
    bounding_box: Rect,
    layout_text: String,
    inline_text_nodes: Option<Vec<InlineTextBox>>,
    style_index: Option<i32>,
}

struct ComputedStyle {
    properties: Vec<NameValue>,
}

struct NameValue {
    name: String,
    value: String,
}

struct DOMSnapshot {
    dom_nodes: Vec<DOMNode>,
    layout_tree_nodes: Vec<LayoutTreeNode>,
    computed_styles: Vec<ComputedStyle>,
}

impl DOMSnapshot {
    pub fn get_snapshot(computed_style_whitelist: Vec<&str>) -> io::Result<Self> {
        unimplemented!()
    }
}
