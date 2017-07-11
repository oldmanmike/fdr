struct Inspector;

impl Inspector {
    fn enable() {
        unimplemented!()
    }
    fn disable() {
        unimplemented!()
    }
}

enum InspectorEvent {
    Detached(String),
    TargetCrashed,
}
