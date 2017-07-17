// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
