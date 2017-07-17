// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

use remote_debugger::runtime::CallFrame;
use remote_debugger::runtime::RemoteObject;
use remote_debugger::runtime::RemoteObjectId;

struct HeapSnapshotObjectId;

struct SamplingHeapProfileNode {
    call_frame: CallFrame,
    self_size: u32,
    children: Vec<SamplingHeapProfileNode>,
}

struct SamplingHeapProfile {
    head: SamplingHeapProfileNode,
}

enum HeapProfilerEvents {
    AddHeapSnapshotChuck(String),
    ResetProfiles,
    ReportHeapSnapshotProgress(i32, i32, bool),
    LastSeenObjectId(i32, u32),
    HeapStatsUpdate(Vec<i32>),
}

struct HeapProfiler;

impl HeapProfiler {
    pub fn enable() {
        unimplemented!()
    }
    pub fn disable() {
        unimplemented!()
    }
    pub fn start_tracking_heap_objects(track_allocations: Option<bool>) {
        unimplemented!()
    }
    pub fn stop_tracking_heap_objects(report_progress: Option<bool>) {
        unimplemented!()
    }
    pub fn take_heap_snapshot(report_progress: Option<bool>) {
        unimplemented!()
    }
    pub fn collect_garbage() {
        unimplemented!()
    }
    pub fn get_object_by_heap_object_id(object_id: HeapSnapshotObjectId,
                                        object_group: Option<&str>)
                                        -> io::Result<RemoteObject> {
        unimplemented!()
    }
    pub fn add_inspected_heap_object(heap_object_id: HeapSnapshotObjectId) {
        unimplemented!()
    }
    pub fn get_heap_object_id(object_id: RemoteObjectId) -> io::Result<HeapSnapshotObjectId> {
        unimplemented!()
    }
    pub fn start_sampling(sampling_interval: Option<u32>) {
        unimplemented!()
    }
    pub fn stop_sampling(profile: SamplingHeapProfile) {
        unimplemented!()
    }
}
