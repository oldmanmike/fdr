// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

pub struct StreamHandle(String);

struct IO;

impl IO {
    pub fn read<'a>(handle: StreamHandle,
                    offset: Option<i32>,
                    size: Option<i32>)
                    -> io::Result<(&'a str, bool)> {
        unimplemented!()
    }
    pub fn close() {
        unimplemented!()
    }
}
