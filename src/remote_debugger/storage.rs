use std::io;

enum StorageType {
    Appcache,
    Cookies,
    FileSystems,
    Indexeddb,
    LocalStorage,
    ShaderCache,
    Websql,
    ServiceWorkers,
    CacheStorage,
    All,
    Other,
}

struct UsageForType {
    storage_type: StorageType,
    usage: u32,
}

struct Usage {
    usage: u32,
    quota: u32,
    usage_breakdown: Vec<UsageForType>,
}

struct Storage;

impl Storage {
    fn clear_data_for_origin(origin: &str, storage_types: &str) {
        unimplemented!()
    }
    fn get_usage_and_quota(origin: &str) -> io::Result<Usage> {
        unimplemented!()
    }
}
