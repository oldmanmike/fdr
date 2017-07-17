// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use remote_debugger::network::RequestId;
use remote_debugger::runtime::StackTrace;
use remote_debugger::runtime::Timestamp;

struct LogEntry {
    source: String,
    level: String,
    text: String,
    timestamp: Timestamp,
    url: Option<String>,
    line_number: Option<i32>,
    stack_trace: Option<StackTrace>,
    network_request_id: RequestId,
    worker_id: Option<String>,
}

struct ViolationSetting {
    name: String,
    threshold: u32,
}

enum LogEvent {
    EntryAdded(LogEntry),
}

struct Log;

impl Log {
    fn enable() {
        unimplemented!()
    }
    fn disable() {
        unimplemented!()
    }
    fn clear() {
        unimplemented!()
    }
    fn start_violations_report(config: Vec<ViolationSetting>) {
        unimplemented!()
    }
    fn stop_violations_report() {
        unimplemented!()
    }
}
