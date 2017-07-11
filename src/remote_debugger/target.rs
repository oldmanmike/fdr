use std::io;

pub struct TargetId(String);

struct BrowserContextId(String);

struct TargetInfo {
    target_id: TargetId,
    target_type: String,
    title: String,
    url: String,
    attached: bool,
}

struct RemoteLocation {
    host: String,
    port: bool,
}

enum TargetEnum {
    TargetCreated { target_info: TargetInfo },
    TargetInfoChanged { target_info: TargetInfo },
    TargetDestroyed { target_id: TargetId },
    AttachedToTarget {
        target_info: TargetInfo,
        waiting_for_debugger: bool,
    },
    DetachedFromTarget { target_id: TargetId },
    ReceivedMessageFromTarget {
        target_id: TargetId,
        message: String,
    },
}

struct Target;

impl Target {
    fn set_discover_targets(discover: bool) {
        unimplemented!()
    }
    fn set_auto_attach(auto_attach: bool, wait_for_debugger_on_start: bool) {
        unimplemented!()
    }
    fn set_attach_to_frames(value: bool) {
        unimplemented!()
    }
    fn set_remote_locations(locations: Vec<RemoteLocation>) {
        unimplemented!()
    }
    fn send_message_to_target(target_id: TargetId, message: &str) {
        unimplemented!()
    }
    fn get_target_info(target_id: TargetId) -> io::Result<TargetInfo> {
        unimplemented!()
    }
    fn activate_target(target_id: TargetId) {
        unimplemented!()
    }
    fn close_target(target_id: TargetId) -> io::Result<bool> {
        unimplemented!()
    }
    fn attach_to_target(target_id: TargetId) -> io::Result<bool> {
        unimplemented!()
    }
    fn detach_from_target(target_id: TargetId) {
        unimplemented!()
    }
    fn create_browser_context() -> io::Result<BrowserContextId> {
        unimplemented!()
    }
    fn dispose_browser_context(browser_context_id: BrowserContextId) -> io::Result<bool> {
        unimplemented!()
    }
    fn create_target(url: &str,
                     width: Option<i32>,
                     height: Option<i32>,
                     browser_context_id: Option<BrowserContextId>)
                     -> io::Result<TargetId> {
        unimplemented!()
    }
    fn get_targets() -> io::Result<Vec<TargetInfo>> {
        unimplemented!()
    }
}
