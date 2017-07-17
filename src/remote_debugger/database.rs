// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

struct DatabaseId(String);

struct SQLResult<A> {
    column_names: Option<Vec<String>>,
    values: Option<Vec<A>>,
    sql_error: Option<DBError>,
}

enum DatabaseEvent {
    AddDatabase(Database),
}

struct Database {
    id: DatabaseId,
    domain: String,
    name: String,
    version: String,
}

struct DBError {
    message: String,
    code: i32,
}

impl Database {
    pub fn enable() {
        unimplemented!()
    }
    pub fn disable() {
        unimplemented!()
    }
    pub fn get_database_table_names<'a>(database_id: DatabaseId) -> io::Result<Vec<&'a str>> {
        unimplemented!()
    }
    pub fn execute_sql<A>(database_id: DatabaseId, query: &str) -> io::Result<SQLResult<A>> {
        unimplemented!()
    }
}
