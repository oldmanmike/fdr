// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

struct Domain {
    name: String,
    version: String,
}

struct Schema;

impl Schema {
    fn get_domains() -> io::Result<Vec<Domain>> {
        unimplemented!()
    }
}
