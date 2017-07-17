// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

enum PressureLevel {
    Moderate,
    Critical,
}

struct Memory;

impl Memory {
    fn get_dom_counters(documents: i32, nodes: i32, js_event_listeners: i32) {
        unimplemented!()
    }
    fn set_pressure_notifications_suppressed(suppressed: bool) {
        unimplemented!()
    }
    fn simulate_pressure_notification(level: PressureLevel) {
        unimplemented!()
    }
}
