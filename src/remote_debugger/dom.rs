use std::io;
use std::sync::mpsc;

use remote_debugger::overlay::HighlightConfig;
use remote_debugger::runtime::RemoteObjectId;
use remote_debugger::runtime::RemoteObject;
use remote_debugger::page::FrameId;
use websocket::{Message, OwnedMessage};

pub struct NodeId(i32);

pub struct BackendNodeId;

pub struct BackendNode {
    node_type: i32,
    node_name: String,
    backend_node_id: BackendNodeId,
}

pub enum PseudoType {
    FirstLine,
    FirstLetter,
    Before,
    After,
    Backdrop,
    Selection,
    FirstLineInherited,
    Scrollbar,
    ScrollbarThumb,
    ScrollbarButton,
    ScrollbarTrack,
    ScrollbarTrackPiece,
    ScrollbarCorner,
    Resizer,
    InputListButton,
}

pub enum ShadowRootType {
    UserAgent,
    Open,
    Closed,
}

pub struct Node {
    node_id: NodeId,
    parent_id: Option<NodeId>,
    backend_node_id: BackendNodeId,
    node_type: i32,
    node_name: String,
    local_name: String,
    node_value: String,
    child_node_count: Option<i32>,
    children: Option<Vec<Node>>,
    attributes: Option<Vec<String>>,
    document_url: Option<String>,
    base_url: Option<String>,
    public_id: Option<String>,
    system_id: Option<String>,
    internal_subset: Option<String>,
    xml_version: Option<String>,
    name: Option<String>,
    value: Option<String>,
    pseudo_type: Option<PseudoType>,
    shadow_root_type: Option<ShadowRootType>,
    frame_id: Option<FrameId>,
    content_document: Option<Box<Node>>,
    shadow_roots: Option<Vec<Node>>,
    template_content: Option<Box<Node>>,
    pseudo_elements: Option<Vec<Node>>,
    imported_document: Option<Box<Node>>,
    distributed_nodes: Option<Vec<BackendNode>>,
    is_svg: Option<bool>,
}

pub struct RGBA {
    r: i32,
    g: i32,
    b: i32,
    a: Option<u32>,
}

pub struct Quad(Vec<(f32, f32)>);

pub struct BoxModel<A> {
    content: Quad,
    padding: Quad,
    border: Quad,
    margin: Quad,
    width: Quad,
    height: Quad,
    shape_outside: Option<ShapeOutsideInfo<A>>,
}

struct ShapeOutsideInfo<A> {
    bounds: Quad,
    shape: Vec<A>,
    margin_shape: Vec<A>,
}

pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

enum DOMEvent {
    DocumentUpdated,
    SetChildNodes(NodeId, Vec<Node>),
    AttributeModified(NodeId, String, String),
    AttributeRemoved(NodeId, String),
    InlineStyleInvalidated(Vec<NodeId>),
    CharacterDataModified(NodeId, String),
    ChildNodeCountUpdated(NodeId, i32),
    ChildNodeInserted(NodeId, NodeId, Node),
    ChildNodeRemoved(NodeId, NodeId),
    ShadowRootPushed(NodeId, Node),
    ShadowRootPopped(NodeId, NodeId),
    PseudoElementAdded(NodeId, Node),
    PseudoElementRemoved(NodeId, NodeId),
    DistributedNodesUpdated(NodeId, Vec<BackendNode>),
}

pub struct DOM {
    conn: mpsc::Sender<OwnedMessage>,
}

impl DOM {
    pub fn enable(self) {
        unimplemented!()
    }
    pub fn disable(self) {
        unimplemented!()
    }
    pub fn get_document(depth: Option<i32>, pierce: Option<bool>) -> io::Result<Node> {
        unimplemented!()
    }
    pub fn get_flattened_document(depth: Option<i32>,
                                  pierce: Option<bool>)
                                  -> io::Result<Vec<Node>> {
        unimplemented!()
    }
    pub fn collect_class_names_from_subtree<'a>(node_id: NodeId) -> io::Result<Vec<&'a str>> {
        unimplemented!()
    }
    pub fn request_child_nodes(node_id: NodeId, depth: Option<i32>, pierce: Option<bool>) {
        unimplemented!()
    }
    pub fn query_selector(node_id: NodeId, selector: &str) -> io::Result<NodeId> {
        unimplemented!()
    }
    pub fn query_selector_all(node_id: NodeId, selector: &str) -> io::Result<Vec<NodeId>> {
        unimplemented!()
    }
    pub fn set_node_name(node_id: NodeId, name: &str) -> io::Result<NodeId> {
        unimplemented!()
    }
    pub fn set_node_value(node_id: NodeId, value: &str) {
        unimplemented!()
    }
    pub fn remove_node(node_id: NodeId) {
        unimplemented!()
    }
    pub fn set_attribute_value(node_id: NodeId, name: &str, value: &str) {
        unimplemented!()
    }
    pub fn set_attribute_as_text(node_id: NodeId, text: &str, name: Option<&str>) {
        unimplemented!()
    }
    pub fn remove_attribute(node_id: NodeId, name: &str) {
        unimplemented!()
    }
    pub fn get_outer_html<'a>(node_id: NodeId) -> io::Result<&'a str> {
        unimplemented!()
    }
    pub fn perform_search(query: &str,
                          include_user_agent_shadow_dom: Option<bool>)
                          -> io::Result<(&str, i32)> {
        unimplemented!()
    }
    pub fn get_search_results(search_id: &str,
                              from_index: i32,
                              to_index: i32)
                              -> io::Result<Vec<NodeId>> {
        unimplemented!()
    }
    pub fn discard_search_results(search_id: &str) {
        unimplemented!()
    }
    pub fn request_node(object_id: RemoteObjectId) -> io::Result<NodeId> {
        unimplemented!()
    }
    pub fn push_node_by_path_to_frontend(path: &str) -> io::Result<NodeId> {
        unimplemented!()
    }
    pub fn push_nodes_by_backend_ids_to_frontend(backend_node_ids: Vec<BackendNodeId>)
                                                 -> io::Result<Vec<NodeId>> {
        unimplemented!()
    }
    pub fn set_inspected_node(node_id: NodeId) {
        unimplemented!()
    }
    pub fn resolve_node(node_id: NodeId, object_group: Option<&str>) -> io::Result<RemoteObject> {
        unimplemented!()
    }
    pub fn get_attributes<'a>(node_id: NodeId) -> io::Result<Vec<&'a str>> {
        unimplemented!()
    }
    pub fn copy_to(node_id: NodeId,
                   target_node_id: NodeId,
                   insert_before_node_id: Option<NodeId>)
                   -> io::Result<NodeId> {
        unimplemented!()
    }
    pub fn move_to(node_id: NodeId,
                   target_node_id: NodeId,
                   insert_before_node_id: Option<NodeId>)
                   -> io::Result<NodeId> {
        unimplemented!()
    }
    pub fn undo() {
        unimplemented!()
    }
    pub fn redo() {
        unimplemented!()
    }
    pub fn mark_undoable_state() {
        unimplemented!()
    }
    pub fn focus(node_id: NodeId) {
        unimplemented!()
    }
    pub fn set_file_input_files<'a>(node_id: NodeId, files: Vec<&'a str>) {
        unimplemented!()
    }
    pub fn get_box_model<A>(node_id: NodeId) -> io::Result<BoxModel<A>> {
        unimplemented!()
    }
    pub fn get_node_for_location(x: i32,
                                 y: i32,
                                 include_user_agent_shadow_dom: Option<bool>)
                                 -> io::Result<NodeId> {
        unimplemented!()
    }
    pub fn get_relayout_boundary(node_id: NodeId) -> io::Result<NodeId> {
        unimplemented!()
    }
}
