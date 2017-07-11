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
