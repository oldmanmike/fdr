struct TouchPoint {
    state: TouchPointState,
    x: i32,
    y: i32,
    radius_x: Option<i32>,
    radius_y: Option<i32>,
    rotation_angle: Option<u32>,
    force: Option<u32>,
    id: Option<u32>,
}

enum TouchPointState {
    TouchPressed,
    TouchReleased,
    TouchMoved,
    TouchStationary,
    TouchCancelled,
}

enum GestureSourceType {
    Default,
    Touch,
    Mouse,
}

struct TimeSinceEpoch(u64);

struct Input;

impl Input {
    fn set_ignore_input_events(ignore: bool) {
        unimplemented!()
    }
    fn dispatch_key_event(event_type: &str,
                          modifiers: Option<i32>,
                          timestamp: Option<TimeSinceEpoch>,
                          text: Option<&str>,
                          unmodified_text: Option<&str>,
                          key_identifier: Option<&str>,
                          code: Option<&str>,
                          key: Option<&str>,
                          windows_virtual_key_code: Option<i32>,
                          native_virtual_key_code: Option<i32>,
                          auto_repeat: Option<bool>,
                          is_keypad: Option<bool>,
                          is_system_key: Option<bool>) {
        unimplemented!()
    }
    fn dispatch_mouse_event(event_type: &str,
                            x: i32,
                            y: i32,
                            modifiers: Option<i32>,
                            timestamp: Option<TimeSinceEpoch>,
                            button: Option<&str>,
                            click_count: Option<i32>) {
        unimplemented!()
    }
    fn dispatch_touch_event(event_type: &str,
                            touch_points: Vec<TouchPoint>,
                            modifiers: Option<i32>,
                            timestamp: Option<TimeSinceEpoch>) {
        unimplemented!()
    }
    fn emulate_touch_from_mouse_event(event_type: &str,
                                      x: i32,
                                      y: i32,
                                      timestamp: TimeSinceEpoch,
                                      button: &str,
                                      delta_x: Option<u32>,
                                      delta_y: Option<u32>,
                                      modifiers: Option<i32>,
                                      click_count: Option<i32>) {
        unimplemented!()
    }
    fn synthesize_pinch_gesture(x: i32,
                                y: i32,
                                scale_factor: u32,
                                relative_speed: Option<i32>,
                                gesture_source_type: Option<GestureSourceType>) {
        unimplemented!()
    }
    fn synthesize_scroll_gesture(x: i32,
                                 y: i32,
                                 x_distance: Option<i32>,
                                 y_distance: Option<i32>,
                                 x_overscroll: Option<i32>,
                                 y_overscroll: Option<i32>,
                                 prevent_fling: Option<bool>,
                                 speed: Option<i32>,
                                 gesture_source_type: Option<GestureSourceType>,
                                 repeat_count: Option<i32>,
                                 repeat_delay_ms: Option<i32>,
                                 interaction_marker_name: Option<&str>) {
        unimplemented!()
    }
}
