use std::io;

use remote_debugger::debugger::SearchMatch;
use remote_debugger::dom::Rect;
use remote_debugger::network::LoaderId;
use remote_debugger::network::TimeSinceEpoch;
use remote_debugger::runtime::ExecutionContextId;

pub enum ResourceType {
    Document,
    Stylesheet,
    Image,
    Media,
    Font,
    Script,
    TextTrack,
    XHR,
    Fetch,
    EventSource,
    WebSocket,
    Manifest,
    Other,
}

pub struct FrameId(String);

struct Frame {
    id: String,
    parent_id: String,
    loader_id: LoaderId,
    name: Option<String>,
    url: String,
    security_origin: String,
    mime_type: String,
}

struct FrameResource {
    url: String,
    resource_type: ResourceType,
    mime_type: String,
    last_modified: Option<TimeSinceEpoch>,
    content_size: Option<u32>,
    failed: Option<bool>,
    canceled: Option<bool>,
}

struct FrameResourceTree {
    frame: Frame,
    child_frames: Option<Vec<FrameResourceTree>>,
    resources: Vec<FrameResource>,
}

struct ScriptIdentifier(String);

enum TransitionType {
    Link,
    Typed,
    AutoBookmark,
    AutoSubframe,
    ManualSubframe,
    Generated,
    AutoToplevel,
    FormSubmit,
    Reload,
    Keyword,
    KeywordGenerated,
    Other,
}

struct NavigationEntry {
    id: i32,
    url: String,
    user_typed_url: String,
    title: String,
    transition_type: TransitionType,
}

struct ScreencastFrameMetadata {
    offset_top: u32,
    page_scale_factor: u32,
    device_width: u32,
    device_height: u32,
    scroll_offset_x: u32,
    scroll_offset_y: u32,
    timestamp: Option<TimeSinceEpoch>,
}

enum DialogType {
    Alert,
    Confirm,
    Prompt,
    Beforeunload,
}

struct AppManifestError {
    message: String,
    critical: i32,
    line: i32,
    column: i32,
}

enum NavigationResponse {
    Proceed,
    Cancel,
    CancelAndIgnore,
}

struct LayoutViewport {
    page_x: i32,
    page_y: i32,
    client_width: i32,
    client_height: i32,
}

struct VisualViewport {
    offset_x: u32,
    offset_y: u32,
    page_x: u32,
    page_y: u32,
    client_width: u32,
    client_height: u32,
    scale: u32,
}

struct Page;

impl Page {
    fn enable() {
        unimplemented!()
    }
    fn disable() {
        unimplemented!()
    }
    fn add_script_to_evaluate_on_load(script_source: &str) -> io::Result<ScriptIdentifier> {
        unimplemented!()
    }
    fn remove_script_to_evaluate_on_load(identifier: ScriptIdentifier) {
        unimplemented!()
    }
    fn add_script_to_evaluate_on_new_document(source: &str) -> io::Result<ScriptIdentifier> {
        unimplemented!()
    }
    fn remove_script_to_evaluate_on_new_document(identifier: ScriptIdentifier) {
        unimplemented!()
    }
    fn set_auto_attach_to_create_pages(auto_attach: bool) {
        unimplemented!()
    }
    fn reload(ignore_cache: Option<bool>, script_to_evaluate_on_load: Option<&str>) {
        unimplemented!()
    }
    fn navigate(url: &str,
                referrer: Option<&str>,
                transition_type: Option<TransitionType>)
                -> io::Result<FrameId> {
        unimplemented!()
    }
    fn stop_loading() {
        unimplemented!()
    }
    fn get_navigation_history(current_index: i32, entries: Vec<NavigationEntry>) {
        unimplemented!()
    }
    fn navigate_to_history_entry(entry_id: i32) {
        unimplemented!()
    }
    fn get_resource_tree() -> io::Result<FrameResourceTree> {
        unimplemented!()
    }
    fn get_resource_content<'a>(frame_id: FrameId, url: &str) -> io::Result<(&'a str, bool)> {
        unimplemented!()
    }
    fn search_in_resource(frame_id: FrameId,
                          url: &str,
                          query: &str,
                          case_sensitive: Option<bool>,
                          is_regex: Option<bool>)
                          -> io::Result<Vec<SearchMatch>> {
        unimplemented!()
    }
    fn set_document_content(frame_id: FrameId, html: &str) {
        unimplemented!()
    }
    fn capture_screenshot(format: Option<&str>,
                          quality: Option<i32>,
                          from_surface: Option<bool>)
                          -> io::Result<String> {
        unimplemented!()
    }
    fn print_to_pdf(landscape: Option<bool>,
                    display_header_footer: Option<bool>,
                    print_background: Option<bool>,
                    scale: Option<u32>,
                    paper_width: Option<u32>,
                    paper_height: Option<u32>,
                    margin_top: Option<u32>,
                    margin_bottom: Option<u32>,
                    margin_left: Option<u32>,
                    margin_right: Option<u32>,
                    page_ranges: Option<&str>)
                    -> io::Result<String> {
        unimplemented!()
    }
    fn start_screencast(format: Option<&str>,
                        quality: Option<i32>,
                        max_width: Option<i32>,
                        max_height: Option<i32>,
                        every_nth_frame: Option<i32>) {
        unimplemented!()
    }
    fn screencast_frame_ack(session_id: i32) {
        unimplemented!()
    }
    fn handle_java_script_dialog(accept: bool, prompt_text: Option<&str>) {
        unimplemented!()
    }
    fn get_app_manifest(url: &str, errors: Vec<AppManifestError>, data: Option<&str>) {
        unimplemented!()
    }
    fn request_app_banner() {
        unimplemented!()
    }
    fn set_control_navigations(enabled: bool) {
        unimplemented!()
    }
    fn process_navigation(response: NavigationResponse, navigation_id: i32) {
        unimplemented!()
    }
    fn get_layout_metrics(layout_viewport: LayoutViewport,
                          visual_viewport: VisualViewport,
                          content_size: Rect) {
        unimplemented!()
    }
    fn create_isolated_world(frame_id: FrameId,
                             world_name: Option<&str>,
                             grant_universal_access: Option<bool>)
                             -> io::Result<ExecutionContextId> {
        unimplemented!()
    }
}
