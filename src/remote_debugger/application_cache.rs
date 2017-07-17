// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

use remote_debugger::page::FrameId;

struct ApplicationCache {
    manifest_url: String,
    size: f32,
    creation_time: f32,
    update_time: f32,
    resourses: Vec<ApplicationCacheResource>,
}

impl ApplicationCache {
    pub fn get_frame_with_manifests() -> io::Result<Vec<FrameWithManifest>> {
        unimplemented!()
    }

    pub fn enable() {
        unimplemented!()
    }

    pub fn get_manifest_for_frame<'a>(frame_id: FrameId) -> io::Result<&'a str> {
        unimplemented!()
    }
}

enum ApplicationCacheEvent {
    ApplicationCacheStatusUpdated(FrameId, String, i32),
    NetworkStateUpdated(bool),
}

struct ApplicationCacheResource {
    url: String,
    size: i32,
    app_cache_type: String,
}

struct FrameWithManifest {
    frame_id: FrameId,
    manifest_url: String,
    status: i32,
}
