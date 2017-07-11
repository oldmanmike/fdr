use std::io;

use remote_debugger::target::TargetId;

struct Browser;

impl Browser {
    pub fn get_window_for_target(target_id: TargetId) -> io::Result<(WindowId, Bounds)> {
        unimplemented!()
    }

    pub fn set_window_bounds(window_id: WindowId, bounds: Bounds) {
        unimplemented!()
    }

    pub fn get_window_bounds(window_id: WindowId) -> io::Result<Bounds> {
        unimplemented!()
    }
}

struct WindowId(i32);

enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

struct Bounds {
    left: Option<i32>,
    top: Option<i32>,
    width: Option<i32>,
    height: Option<i32>,
    window_state: Option<WindowState>,
}
