use std::io;

use remote_debugger::dom::BackendNodeId;
use remote_debugger::dom::NodeId;
use remote_debugger::dom::Quad;
use remote_debugger::dom::RGBA;
use remote_debugger::page::FrameId;
use remote_debugger::runtime::RemoteObjectId;

pub struct HighlightConfig {
    show_info: Option<bool>,
    show_rulers: Option<bool>,
    show_extensions_lines: Option<bool>,
    display_as_material: Option<bool>,
    content_color: Option<RGBA>,
    padding_color: Option<RGBA>,
    border_color: Option<RGBA>,
    margin_color: Option<RGBA>,
    event_target_Color: Option<RGBA>,
    shape_color: Option<RGBA>,
    shape_margin_color: Option<RGBA>,
    selector_list: Option<String>,
}

enum InspectMode {
    SearchForNode,
    SearchForUAShadowDOM,
    None,
}

enum OverlayEvent {
    NodeHighlightRequested { node_id: NodeId },
    InspectNodeRequested { backend_node_id: BackendNodeId },
}

struct Overlay;

impl Overlay {
    fn enable() {
        unimplemented!()
    }
    fn disable() {
        unimplemented!()
    }
    fn set_show_paint_rects(result: bool) {
        unimplemented!()
    }
    fn set_show_debug_borders(show: bool) {
        unimplemented!()
    }
    fn set_show_fps_counter(show: bool) {
        unimplemented!()
    }
    fn set_show_scroll_bottleneck_rects(show: bool) {
        unimplemented!()
    }
    fn set_show_viewport_size_on_resize(show: bool) {
        unimplemented!()
    }
    fn set_paused_in_debugger_message(message: Option<&str>) {
        unimplemented!()
    }
    fn set_suspended(suspended: bool) {
        unimplemented!()
    }
    fn set_inspect_mode(mode: InspectMode, highlight_config: Option<HighlightConfig>) {
        unimplemented!()
    }
    fn highlight_rect(x: i32,
                      y: i32,
                      width: i32,
                      height: i32,
                      color: Option<RGBA>,
                      outline_color: Option<RGBA>) {
        unimplemented!()
    }
    fn highlight_quad(quad: Quad, color: Option<RGBA>, outline_color: Option<RGBA>) {
        unimplemented!()
    }
    fn highlight_node(highlight_config: HighlightConfig,
                      node_id: Option<BackendNodeId>,
                      object_id: Option<RemoteObjectId>) {
        unimplemented!()
    }
    fn highlight_frame(frame_id: FrameId,
                       content_color: Option<RGBA>,
                       content_outline_color: Option<RGBA>) {
        unimplemented!()
    }
    fn hide_highlight() {
        unimplemented!()
    }
    fn get_highlight_object_for_test(node_id: NodeId) -> io::Result<String> {
        unimplemented!()
    }
}
