// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

use remote_debugger::debugger::Location;
use remote_debugger::runtime::CallFrame;
use remote_debugger::runtime::ScriptId;

struct ProfileNode {
    id: i32,
    call_frame: CallFrame,
    hit_count: Option<i32>,
    children: Option<Vec<i32>>,
    deopt_reason: Option<String>,
    position_ticks: Option<Vec<PositionTickInfo>>,
}

struct Profile {
    nodes: Vec<ProfileNode>,
    start_time: u32,
    end_time: u32,
    samples: Option<Vec<i32>>,
    time_deltas: Option<Vec<i32>>,
}

struct PositionTickInfo {
    line: i32,
    ticks: i32,
}

struct CoverageRange {
    start_offset: i32,
    end_offset: i32,
    count: i32,
}

struct FunctionCoverage {
    function_name: String,
    ranges: Vec<CoverageRange>,
    is_block_coverage: bool,
}

struct ScriptCoverage {
    script_id: ScriptId,
    url: String,
    functions: Vec<FunctionCoverage>,
}

enum ProfileEvent {
    ConsoleProfileStarted {
        id: String,
        location: Location,
        title: Option<String>,
    },
    ConsoleProfileFinished {
        id: String,
        location: Location,
        profile: Profile,
        title: Option<String>,
    },
}

struct Profiler;

impl Profiler {
    fn enable() {
        unimplemented!()
    }
    fn disable() {
        unimplemented!()
    }
    fn set_sampling_interval(interval: i32) {
        unimplemented!()
    }
    fn start() {
        unimplemented!()
    }
    fn stop(profile: Profile) {
        unimplemented!()
    }
    fn start_precise_coverage(call_count: Option<bool>) {
        unimplemented!()
    }
    fn stop_precise_coverage() {
        unimplemented!()
    }
    fn take_precise_coverage() -> io::Result<Vec<ScriptCoverage>> {
        unimplemented!()
    }
    fn get_best_effort_coverage() -> io::Result<Vec<ScriptCoverage>> {
        unimplemented!()
    }
}
