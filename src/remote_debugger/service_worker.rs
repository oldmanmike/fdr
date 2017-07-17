// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

use remote_debugger::target::TargetId;

struct ServiceWorkerRegistration {
    registration_id: String,
    scope_url: String,
    is_deleted: bool,
}

enum ServiceWorkerVersionRunningStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
}

enum ServiceWorkerVersionStatus {
    New,
    Installing,
    Installed,
    Activating,
    Activated,
    Redundant,
}

struct ServiceWorkerVersion {
    version_id: String,
    registration_id: String,
    script_url: String,
    running_status: ServiceWorkerVersionRunningStatus,
    status: ServiceWorkerVersionStatus,
    script_last_modified: Option<u32>,
    script_response_time: Option<u32>,
    controlled_clients: Option<Vec<TargetId>>,
    target_id: Option<TargetId>,
}

struct ServiceWorkerErrorMessage {
    error_message: String,
    registration_id: String,
    version_id: String,
    source_url: String,
    line_number: i32,
    column_number: i32,
}

enum ServiceWorkerEvent {
    WorkerRegistrationUpdated { registrations: Vec<ServiceWorkerRegistration>, },
    WorkerVersionUpdated { versions: Vec<ServiceWorkerVersion> },
    WorkerErrorReported { error_message: ServiceWorkerErrorMessage, },
}

struct ServiceWorker;

impl ServiceWorker {
    fn enable() {
        unimplemented!()
    }
    fn disable() {
        unimplemented!()
    }
    fn unregister(scope_url: &str) {
        unimplemented!()
    }
    fn update_registration(scope_url: &str) {
        unimplemented!()
    }
    fn start_worker(scope_url: &str) {
        unimplemented!()
    }
    fn skip_waiting(scope_url: &str) {
        unimplemented!()
    }
    fn stop_worker(version_id: &str) {
        unimplemented!()
    }
    fn inspect_worker(version_id: &str) {
        unimplemented!()
    }
    fn set_force_update_on_page_load(force_update_on_page_load: bool) {
        unimplemented!()
    }
    fn deliver_push_message(origin: &str, registration_id: &str, data: &str) {
        unimplemented!()
    }
    fn dispatch_sync_event(origin: &str, registration_id: &str, tag: &str, last_chance: bool) {
        unimplemented!()
    }
}
