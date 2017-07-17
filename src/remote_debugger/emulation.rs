// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

use remote_debugger::dom::RGBA;

enum EmulationEvent {
    VirtualTimeBudgetExpired,
}

struct ScreenOrientation {
    orient_type: OrientType,
    angle: i32,
}

enum OrientType {
    PortraitPrimary,
    PortraitSecondary,
    LandscapePrimary,
    LandscapeSecondary,
}

enum VirtualTimePolicy {
    Advance,
    Pause,
    PauseIfNetworkFetchesPending,
}

struct Emulation;

impl Emulation {
    pub fn set_device_metrics_override(width: i32,
                                       height: i32,
                                       device_scale_factor: u32,
                                       mobile: bool,
                                       fit_window: bool,
                                       scale: Option<u32>,
                                       offset_x: Option<u32>,
                                       offset_y: Option<u32>,
                                       screen_width: Option<i32>,
                                       screen_height: Option<i32>,
                                       position_x: Option<i32>,
                                       position_y: Option<i32>,
                                       screen_orientation: Option<ScreenOrientation>) {
        unimplemented!()
    }
    pub fn clear_device_metrics_override() {
        unimplemented!()
    }
    pub fn force_viewport(x: u32, y: u32, scale: u32) {
        unimplemented!()
    }
    pub fn reset_viewport() {
        unimplemented!()
    }
    pub fn reset_page_scale_factor() {
        unimplemented!()
    }
    pub fn set_page_scale_factor(page_scale_factor: u32) {
        unimplemented!()
    }
    pub fn set_visible_size(width: u32, height: u32) {
        unimplemented!()
    }
    pub fn set_script_execution_disabled(value: bool) {
        unimplemented!()
    }
    pub fn set_geolocation_override(latitude: Option<u32>,
                                    longitude: Option<u32>,
                                    accuracy: Option<u32>) {
        unimplemented!()
    }
    pub fn clear_geolocation_override() {
        unimplemented!()
    }
    pub fn set_touch_emulation_enabled(enabled: bool, config: Option<String>) {
        unimplemented!()
    }
    pub fn set_emulated_media(media: &str) {
        unimplemented!()
    }
    pub fn set_cpu_throttling_rate(rate: u32) {
        unimplemented!()
    }
    pub fn can_emulate() -> io::Result<bool> {
        unimplemented!()
    }
    pub fn set_virtual_time_policy(policy: VirtualTimePolicy, budget: Option<i32>) {
        unimplemented!()
    }
    pub fn set_default_background_color_override(color: Option<RGBA>) {
        unimplemented!()
    }
}
