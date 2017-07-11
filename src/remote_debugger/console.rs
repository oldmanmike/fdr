use std::io;

struct Console;

impl Console {
    pub fn enable() {
        unimplemented!()
    }

    pub fn disable() {
        unimplemented!()
    }

    pub fn clear_messages() {
        unimplemented!()
    }
}

enum ConsoleEvent {
    MessageAdded(ConsoleMessage),
}

struct ConsoleMessage {
    source: String,
    level: String,
    text: String,
    url: Option<String>,
    line: Option<i32>,
    column: Option<i32>,
}
