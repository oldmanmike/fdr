// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

use remote_debugger::runtime::CallArgument;
use remote_debugger::runtime::ExecutionContextId;
use remote_debugger::runtime::ExceptionDetails;
use remote_debugger::runtime::RemoteObject;
use remote_debugger::runtime::ScriptId;
use remote_debugger::runtime::StackTrace;

struct BreakpointId(String);

struct CallFrameId(String);

pub struct Location {
    script_id: ScriptId,
    line_number: i32,
    column_number: Option<i32>,
}

struct ScriptPosition {
    line_number: i32,
    column_number: i32,
}

struct CallFrame {
    call_frame_id: CallFrameId,
    function_name: String,
    function_location: Option<Location>,
    location: Location,
    scope_chain: Vec<Scope>,
    this: RemoteObject,
    return_value: Option<RemoteObject>,
}

struct Scope {
    scope_type: ScopeType,
    object: RemoteObject,
    name: Option<String>,
}

enum ScopeType {
    Global,
    Local,
    With,
    Closure,
    Catch,
    Block,
    Script,
    Eval,
    Module,
}

pub struct SearchMatch {
    line_number: u32,
    line_content: u32,
}

struct BreakLocation {
    script_id: ScriptId,
    line_number: i32,
    column_number: Option<i32>,
    break_type: BreakType,
}

enum BreakType {
    DebuggerStatement,
    Call,
    Return,
}

struct LiveStackDetails {
    call_frames: Option<Vec<CallFrame>>,
    stack_changed: Option<bool>,
    async_stack_trace: Option<StackTrace>,
    exception_details: Option<ExceptionDetails>,
}

struct NewStackDetails {
    call_frames: Vec<CallFrame>,
    async_stack_trace: Option<StackTrace>,
}

enum DebuggerEvent<A> {
    ScriptParsed(ScriptId,
                 String,
                 i32,
                 i32,
                 i32,
                 i32,
                 ExecutionContextId,
                 String,
                 A,
                 Option<bool>,
                 Option<String>,
                 Option<bool>,
                 Option<bool>,
                 Option<i32>,
                 Option<StackTrace>),
    ScriptFailedToParse(ScriptId,
                        String,
                        i32,
                        i32,
                        i32,
                        i32,
                        ExecutionContextId,
                        String,
                        A,
                        Option<bool>,
                        Option<String>,
                        Option<bool>,
                        Option<bool>,
                        Option<i32>,
                        Option<StackTrace>),
    BreakpointResolved(BreakpointId, Location),
    Paused(Vec<CallFrame>, String, Option<A>, StackTrace),
    Resumed,
}

struct Debugger;

impl Debugger {
    pub fn enable() {
        unimplemented!()
    }
    pub fn disable() {
        unimplemented!()
    }
    pub fn set_breakpoints_active(active: bool) {
        unimplemented!()
    }
    pub fn set_skip_all_pauses(skip: bool) {
        unimplemented!()
    }
    pub fn set_breakpoint_by_url(line_number: i32,
                                 url: Option<&str>,
                                 url_regex: Option<&str>,
                                 column_number: Option<&str>,
                                 condition: Option<&str>)
                                 -> io::Result<(BreakpointId, Vec<Location>)> {
        unimplemented!()
    }
    pub fn set_breakpoint(location: Location,
                          condition: Option<&str>)
                          -> io::Result<(BreakpointId, Location)> {
        unimplemented!()
    }
    pub fn remove_breakpoint(breakpoint_id: BreakpointId) {
        unimplemented!()
    }
    pub fn get_possible_breakpoints(start: Location,
                                    end: Option<Location>,
                                    restrict_to_function: Option<bool>)
                                    -> io::Result<Vec<BreakLocation>> {
        unimplemented!()
    }
    pub fn continue_to_location(location: Location, target_call_frames: Option<&str>) {
        unimplemented!()
    }
    pub fn step_over() {
        unimplemented!()
    }
    pub fn step_into() {
        unimplemented!()
    }
    pub fn step_out() {
        unimplemented!()
    }
    pub fn pause() {
        unimplemented!()
    }
    pub fn schedule_step_into_async() {
        unimplemented!()
    }
    pub fn resume() {
        unimplemented!()
    }
    pub fn search_in_content(script_id: ScriptId,
                             query: &str,
                             case_sensitive: Option<bool>,
                             is_regex: Option<bool>)
                             -> io::Result<Vec<SearchMatch>> {
        unimplemented!()
    }
    pub fn set_script_source(script_id: ScriptId,
                             script_source: &str,
                             dry_run: Option<bool>)
                             -> io::Result<LiveStackDetails> {
        unimplemented!()
    }
    pub fn restart_frame(call_frame_id: CallFrameId) -> io::Result<NewStackDetails> {
        unimplemented!()
    }
    pub fn get_script_source<'a>(script_id: ScriptId) -> io::Result<&'a str> {
        unimplemented!()
    }
    pub fn set_pause_on_exceptions(state: &str) {
        unimplemented!()
    }
    pub fn evaluate_on_call_frame(call_frame_id: CallFrameId,
                                  expression: &str,
                                  object_group: Option<&str>,
                                  include_command_line_api: Option<bool>,
                                  silent: Option<bool>,
                                  return_by_value: Option<bool>,
                                  generate_preview: Option<bool>,
                                  throw_on_side_effect: Option<bool>)
                                  -> io::Result<(RemoteObject, ExceptionDetails)> {
        unimplemented!()
    }
    pub fn set_variable_value(scope_number: i32,
                              variable_name: &str,
                              new_value: CallArgument,
                              call_frame_id: CallFrameId) {
        unimplemented!()
    }
    pub fn set_async_call_stack_depth(max_depth: i32) {
        unimplemented!()
    }
    pub fn set_blackbox_patterns(patterns: Vec<&str>) {
        unimplemented!()
    }
    pub fn set_blackboxed_ranges(script_id: ScriptId, positions: Vec<ScriptPosition>) {
        unimplemented!()
    }
}
