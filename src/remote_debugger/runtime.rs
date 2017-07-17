// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

pub struct ScriptId(String);

pub struct RemoteObjectId(String);

enum UnserializableValue {
    Infinity,
    NaN,
    NegInfinity,
    NegZero,
}

pub struct RemoteObject {
    object_type: ObjectType,
    subtype: Option<ObjectSubtype>,
    class_name: Option<String>,
    value: Option<String>,
    unserializable_value: Option<UnserializableValue>,
    description: Option<String>,
    object_id: Option<RemoteObjectId>,
    preview: Option<ObjectPreview>,
    custom_preview: Option<CustomPreview>,
}

struct CustomPreview {
    header: String,
    has_body: bool,
    formatter_object_id: RemoteObjectId,
    bind_remote_object_function_id: RemoteObjectId,
    config_object_id: Option<RemoteObjectId>,
}

struct ObjectPreview {
    object_type: ObjectType,
    subtype: Option<ObjectSubtype>,
    description: Option<String>,
    overflow: bool,
    properties: Vec<PropertyPreview>,
    entries: Option<Vec<EntryPreview>>,
}

struct PropertyPreview {
    name: String,
    object_type: ObjectType,
    value: Option<String>,
    value_preview: Option<ObjectPreview>,
    subtype: Option<String>,
}

struct EntryPreview {
    key: Option<ObjectPreview>,
    value: ObjectPreview,
}

struct PropertyDescriptor {
    name: String,
    value: Option<RemoteObject>,
    writable: Option<bool>,
    get: Option<RemoteObject>,
    set: Option<RemoteObject>,
    configurable: bool,
    enumerable: bool,
    was_thrown: Option<bool>,
    is_own: Option<bool>,
    symbol: Option<RemoteObject>,
}

struct InternalPropertyDescriptor {
    name: String,
    value: Option<RemoteObject>,
}

pub struct CallArgument {
    value: Option<String>,
    unserializable_value: Option<UnserializableValue>,
    object_id: Option<RemoteObjectId>,
}

pub struct ExecutionContextId(i32);

struct ExecutionContextDescription {
    id: ExecutionContextId,
    origin: String,
    name: String,
    aux_data: Option<String>, // object
}

pub struct ExceptionDetails {
    exception_id: i32,
    text: String,
    line_number: i32,
    column_number: i32,
    script_id: Option<ScriptId>,
    url: Option<String>,
    stack_trace: Option<StackTrace>,
    exception: Option<RemoteObject>,
    execution_context_id: Option<ExecutionContextId>,
}

pub struct Timestamp(u64);

pub struct CallFrame {
    function_name: String,
    script_id: ScriptId,
    url: String,
    line_number: i32,
    column_number: i32,
}

pub struct StackTrace {
    description: Option<String>,
    call_frames: Option<Vec<CallFrame>>,
    parent: Box<Option<StackTrace>>,
    promise_creation_frame: Option<CallFrame>,
}

enum ObjectType {
    Object,
    Function,
    Undefined,
    StringObject,
    Number,
    Boolean,
    Symbol,
}

enum ObjectSubtype {
    ArrayObject,
    NullObject,
    NodeObject,
    RegexpObject,
    DateObject,
    MapObject,
    SetObject,
    WeakmapObject,
    WeaksetObject,
    IteratorObject,
    GeneratorObject,
    ErrorObject,
    ProxyObject,
    PromiseObject,
    TypedarrayObject,
}

struct Runtime;

impl Runtime {
    fn evaluate(expr: &str,
                object_group: Option<&str>,
                include_command_line_api: Option<bool>,
                silent: Option<bool>,
                context_id: Option<ExecutionContextId>,
                return_by_value: Option<bool>,
                generate_preview: Option<bool>,
                user_gesture: Option<bool>,
                await_promise: Option<bool>)
                -> io::Result<(RemoteObject, Option<ExceptionDetails>)> {
        unimplemented!()
    }
    fn await_promise(promise_object_id: RemoteObjectId,
                     return_by_value: Option<bool>,
                     generate_preview: Option<bool>)
                     -> io::Result<(RemoteObject, Option<ExceptionDetails>)> {
        unimplemented!()
    }
    fn call_function_on(object_id: RemoteObjectId,
                        function_declaration: &str,
                        arguments: Option<Vec<CallArgument>>,
                        silent: Option<bool>,
                        return_by_value: Option<bool>,
                        generate_preview: Option<bool>,
                        user_gesture: Option<bool>,
                        await_promise: Option<bool>)
                        -> io::Result<(RemoteObject, ExceptionDetails)> {
        unimplemented!()
    }
    fn get_properties(object_id: RemoteObjectId,
                      own_properties: Option<bool>,
                      accessor_properties_only: Option<bool>,
                      generate_preview: Option<bool>)
                      -> io::Result<PropertiesResult> {
        unimplemented!()
    }
}

struct PropertiesResult {
    result: Vec<PropertyDescriptor>,
    internal_properties: Option<Vec<InternalPropertyDescriptor>>,
    exception_details: Option<ExceptionDetails>,
}
