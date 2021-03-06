// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

struct CacheStorage;

impl CacheStorage {
    pub fn request_cache_names<'a>(security_origin: &'a str) -> io::Result<Vec<Cache>> {
        unimplemented!()
    }

    pub fn request_entities(cache_id: CacheId,
                            skip_count: i32,
                            page_size: i32)
                            -> io::Result<(Vec<DataEntry>, bool)> {
        unimplemented!()
    }

    pub fn delete_cache(cache_id: CacheId) {
        unimplemented!()
    }

    pub fn delete_entry<'a>(cache_id: CacheId, request: &'a str) {
        unimplemented!()
    }
}

struct CacheId(String);

struct DataEntry {
    request: String,
    response: String,
    response_time: String,
}

struct Cache {
    cache_id: CacheId,
    security_origin: String,
    cache_name: String,
}
