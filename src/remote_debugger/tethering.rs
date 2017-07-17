// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

enum TetheringEvent {
    Accepted(i32, String),
}

struct Tethering;

impl Tethering {
    fn bind(port: i32) {
        unimplemented!()
    }
    fn unbind(port: i32) {
        unimplemented!()
    }
}
