// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

struct Console;

impl Console {
    pub fn enable() {
        unimplemented!()
    }

    pub fn disable() {
        unimplemented!()
    }

    pub fn clear_messages() {
        unimplemented!()
    }
}

enum ConsoleEvent {
    MessageAdded(ConsoleMessage),
}

struct ConsoleMessage {
    source: String,
    level: String,
    text: String,
    url: Option<String>,
    line: Option<i32>,
    column: Option<i32>,
}
