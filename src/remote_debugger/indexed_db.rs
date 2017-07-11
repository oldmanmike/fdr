use std::io;

use remote_debugger::runtime::RemoteObject;

struct DatabaseWithObjectStores {
    name: String,
    version: i32,
    object_stores: Vec<ObjectStore>,
}

struct ObjectStore {
    name: String,
    key_path: KeyPath,
    auto_increment: bool,
    indexes: Vec<ObjectStoreIndex>,
}

struct ObjectStoreIndex {
    name: String,
    key_path: KeyPath,
    unique: bool,
    multi_entry: bool,
}

struct Key {
    key_type: String,
    number: Option<u32>,
    string: Option<String>,
    date: Option<i32>,
    array: Option<Vec<Key>>,
}

struct KeyRange {
    lower: Option<Key>,
    upper: Option<Key>,
    lower_open: bool,
    upper_open: bool,
}

struct DataEntry {
    key: RemoteObject,
    primary_key: RemoteObject,
    value: RemoteObject,
}

struct KeyPath {
    key_type: String,
    string: Option<String>,
    array: Option<Vec<String>>,
}

struct IndexedDB;

impl IndexedDB {
    pub fn enable() {
        unimplemented!()
    }
    pub fn disable() {
        unimplemented!()
    }
    pub fn request_database_names<'a>(security_origin: &str) -> io::Result<Vec<&'a str>> {
        unimplemented!()
    }
    pub fn request_database(security_origin: &str,
                            database_name: &str)
                            -> io::Result<DatabaseWithObjectStores> {
        unimplemented!()
    }
    pub fn request_data(security_origin: &str,
                        database_name: &str,
                        object_store_name: &str,
                        index_name: &str,
                        skip_count: i32,
                        page_size: i32,
                        key_range: Option<KeyRange>)
                        -> io::Result<(Vec<DataEntry>, bool)> {
        unimplemented!()
    }
    pub fn clear_object_store(security_origin: &str,
                              database_name: &str,
                              object_store_name: &str) {
        unimplemented!()
    }
    pub fn delete_database(security_origin: &str, database_name: &str) {
        unimplemented!()
    }
}
