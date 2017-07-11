use std::io;

use remote_debugger::io::StreamHandle;

enum TraceEvent<A> {
    DataCollected(Vec<A>),
    TracingComplete(Option<StreamHandle>),
    BufferUsage(Option<u32>, Option<u32>, Option<u32>),
}

struct MemoryDumpConfig;

struct TraceConfig {
    record_mode: Option<String>,
    enable_sampling: Option<bool>,
    enable_systrace: Option<bool>,
    enable_argument_filter: Option<bool>,
    included_categories: Option<Vec<String>>,
    excluded_categories: Option<Vec<String>>,
    synthetic_delays: Option<String>,
    memory_dump_config: Option<MemoryDumpConfig>,
}

struct Trace;

impl Trace {
    fn start(categories: Option<&str>,
             options: Option<&str>,
             buffer_usage_reporting_interval: Option<u32>,
             transfer_mode: Option<&str>,
             trace_config: Option<TraceConfig>) {
        unimplemented!()
    }
    fn end() {
        unimplemented!()
    }
    fn get_categories(categories: Vec<&str>) {
        unimplemented!()
    }
    fn request_memory_dump<'a>() -> io::Result<(&'a str, bool)> {
        unimplemented!()
    }
    fn record_clock_sync_marker(sync_id: &str) {
        unimplemented!()
    }
}
