// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

struct GPUDevice {
    vendor_id: u32,
    device_id: u32,
    vendor_string: String,
    device_string: String,
}

struct GPUInfo {
    devices: Vec<GPUDevice>,
    aux_attributes: Option<String>,
    features_status: Option<String>,
    driver_bug_workarounds: Vec<String>,
}

struct SystemInfo;

impl SystemInfo {
    fn get_info(gpu: GPUInfo, model_name: &str, model_version: &str, command_line: &str) {
        unimplemented!()
    }
}
