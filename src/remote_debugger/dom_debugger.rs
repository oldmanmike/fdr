use std::io;

use remote_debugger::dom::BackendNodeId;
use remote_debugger::dom::NodeId;
use remote_debugger::runtime::RemoteObject;
use remote_debugger::runtime::RemoteObjectId;
use remote_debugger::runtime::ScriptId;

struct DOMDebugger;

impl DOMDebugger {
    pub fn set_dom_breakpoint(node_id: NodeId, breakpoint_type: DOMBreakpointType) {
        unimplemented!()
    }
    pub fn remove_dom_breakpoint(node_id: NodeId, breakpoint_type: DOMBreakpointType) {
        unimplemented!()
    }
    pub fn set_event_listener_breakpoint(event_name: &str, target_name: Option<&str>) {
        unimplemented!()
    }
    pub fn remove_event_listener_breakpoint(event_name: &str, target_name: Option<&str>) {
        unimplemented!()
    }
    pub fn set_instrumentation_breakpoint(event_name: &str) {
        unimplemented!()
    }
    pub fn remove_instrumentation_breakpoint(event_name: &str) {
        unimplemented!()
    }
    pub fn set_xhr_breakpoint(url: &str) {
        unimplemented!()
    }
    pub fn remove_xhr_breakpoint(url: &str) {
        unimplemented!()
    }
    pub fn get_event_listeners(object_id: RemoteObjectId, depth: Option<i32>, pierce: Option<bool>) -> io::Result<Vec<EventListener>> {
        unimplemented!()
    }
}

pub enum DOMBreakpointType {
    SubtreeModified,
    AttributeModified,
    NodeRemoved,
}

struct EventListener {
    listener_type: String,
    use_capture: bool,
    passive: bool,
    once: bool,
    script_id: ScriptId,
    line_number: i32,
    column_number: i32,
    handler: Option<RemoteObject>,
    original_handler: Option<RemoteObject>,
    backend_node_id: Option<BackendNodeId>,
}
