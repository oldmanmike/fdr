use std::io;

struct DOMItem;

struct StorageId {
    security_origin: String,
    is_local_storage: bool,
}

enum DOMStorageEvent {
    DOMStorageItemsCleared(StorageId),
    DOMStorageItemRemoved(StorageId, String),
    DOMStorageItemAdded(StorageId, String, String),
    DOMStorageItemUpdated(StorageId, String, String, String),
}

struct DOMStorage;

impl DOMStorage {
    pub fn enable() {
        unimplemented!()
    }
    pub fn disable() {
        unimplemented!()
    }
    pub fn clear() {
        unimplemented!()
    }
    pub fn get_dom_storage_items(storage_id: StorageId) -> io::Result<Vec<DOMItem>> {
        unimplemented!()
    }
    pub fn get_dom_storage_item(storage_id: StorageId, key: &str, value: &str) {
        unimplemented!()
    }
    pub fn remove_dom_storage_item(storage_id: StorageId, key: &str) {
        unimplemented!()
    }
}
