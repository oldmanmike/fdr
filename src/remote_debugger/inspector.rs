// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

struct Inspector;

impl Inspector {
    fn enable() {
        unimplemented!()
    }
    fn disable() {
        unimplemented!()
    }
}

enum InspectorEvent {
    Detached(String),
    TargetCrashed,
}
