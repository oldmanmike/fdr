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
