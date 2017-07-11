use std::io;

use remote_debugger::dom::BackendNodeId;
use remote_debugger::runtime::RemoteObject;

struct Animation {
    id: String,
    name: String,
    paused_state: bool,
    play_state: String,
    playback_rate: f32,
    start_time: f32,
    current_time: f32,
    source: AnimationEffect,
    animation_type: AnimationType,
    css_id: Option<String>,
}

impl Animation {
    pub fn enable() {
        unimplemented!()
    }

    pub fn disable() {
        unimplemented!()
    }

    pub fn get_playback_rate() -> io::Result<f32> {
        unimplemented!()
    }

    pub fn set_playback_rate(playback_rate: f32) {
        unimplemented!()
    }

    pub fn get_current_time(id: &str) -> io::Result<f32> {
        unimplemented!()
    }

    pub fn set_paused<'a>(animations: Vec<&'a str>, paused: bool) {
        unimplemented!()
    }

    pub fn set_timing(animation_id: &str, duration: f32, delay: f32) {
        unimplemented!()
    }

    pub fn seek_animations<'a>(animations: Vec<&'a str>, current_time: f32) {
        unimplemented!()
    }

    pub fn release_animations<'a>(animations: Vec<&'a str>) {
        unimplemented!()
    }

    pub fn resolved_animation(animation_id: &str) -> io::Result<RemoteObject> {
        unimplemented!()
    }
}

enum AnimationType {
    CSSTransition,
    CSSAnimation,
    WebAnimation,
}

enum AnimationEvent {
    AnimationCreated(String),
    AnimationStarted(Animation),
    AnimationCanceled(String),
}

struct AnimationEffect {
    delay: f32,
    end_delay: f32,
    iteration_start: f32,
    iterations: f32,
    duration: f32,
    direction: String,
    fill: String,
    backend_node_id: BackendNodeId,
    keyframes_rule: Option<KeyframesRule>,
    easing: String,
}

struct KeyframesRule {
    name: Option<String>,
    keyframes: Vec<KeyframeStyle>,
}

struct KeyframeStyle {
    offset: String,
    easing: String,
}
