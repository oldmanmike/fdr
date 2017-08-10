// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate inflections;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use inflections::case;
use std::fs::File;
use std::io::{Read, Write, BufWriter};

#[derive(Debug, Deserialize)]
struct ExtractedCDP {
    version: ExtractedCDPVersion,
    domains: Vec<ExtractedDomain>,
}

#[derive(Debug, Deserialize, Clone)]
struct ExtractedCDPVersion {
    major: String,
    minor: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct ExtractedDomain {
    domain: String,
    description: Option<String>,
    dependencies: Option<Vec<String>>,
    deprecated: Option<bool>,
    experimental: Option<bool>,
    #[serde(rename = "types")]
    extracted_types: Option<Vec<ExtractedType>>,
    commands: Option<Vec<ExtractedCommand>>,
    events: Option<Vec<ExtractedEvent>>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct ExtractedType {
    id: String,
    #[serde(rename = "type")]
    extracted_type: String,
    description: Option<String>,
    properties: Option<Vec<ExtractedStructField>>,
    experimental: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct ExtractedStructField {
    name: String,
    #[serde(rename = "type")]
    extracted_type: Option<String>,
    #[serde(rename = "$ref")]
    extracted_ref: Option<String>,
    optional: Option<bool>,
    experimental: Option<bool>,
    #[serde(rename = "enum")]
    extracted_enum: Option<Vec<String>>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct ExtractedCommand {
    name: String,
    parameters: Option<Vec<ExtractedCmdField>>,
    returns: Option<Vec<ExtractedReturnType>>,
    description: Option<String>,
    experimental: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct ExtractedCmdField {
    name: String,
    extracted_type: Option<String>,
    extracted_ref: Option<String>,
    optional: Option<bool>,
    experimental: Option<bool>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct ExtractedReturnType {
    name: String,
    extracted_type: Option<String>,
    extracted_ref: Option<String>,
    description: Option<String>,
    optional: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct ExtractedEvent {
    name: String,
    parameters: Option<Vec<ExtractedEventField>>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct ExtractedEventField {
    name: String,
    #[serde(rename = "type")]
    extracted_type: Option<String>,
    #[serde(rename = "$ref")]
    extracted_ref: Option<String>,
    items: Option<ExtractedEventFieldItem>,
    #[serde(rename = "enum")]
    extracted_enum: Option<Vec<String>>,
    optional: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum EventItem {
    Singular(ExtractedEventFieldItem),
    Plural(Vec<ExtractedEventFieldItem>),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ExtractedEventFieldItem {
    #[serde(rename = "type")]
    extracted_type: Option<String>,
    #[serde(rename = "$ref")]
    extracted_ref: Option<String>,
}

fn main() {
    println!("Generating Chrome Devtools Protocol bindings...");

    let mut resp1 = reqwest::get("http://raw.githubusercontent.\
                      com/ChromeDevTools/devtools-protocol/master/json/browser_protocol.json")
        .unwrap();
    assert!(resp1.status().is_success());
    println!("browser_protocol has arrived!");
    let mut browser_content = String::new();
    resp1.read_to_string(&mut browser_content);
    let browser_proto: ExtractedCDP = serde_json::from_str(&browser_content).unwrap();

    let mut resp2 = reqwest::get("http://raw.githubusercontent.\
                      com/ChromeDevTools/devtools-protocol/master/json/js_protocol.json")
        .unwrap();
    assert!(resp2.status().is_success());
    println!("js_protocol has arrived!");
    let mut js_content = String::new();
    resp2.read_to_string(&mut js_content);
    let js_proto: ExtractedCDP = serde_json::from_str(&js_content).unwrap();

    println!("Browser Protocol Version: {:?}", browser_proto.version);
    println!("JS Protocol Version: {:?}", js_proto.version);
    let mut domains = browser_proto.domains;
    domains.extend(js_proto.domains.iter().cloned());
    domains.sort();
    mk_lib(&domains);
    for d in domains.iter() {
        mk_domain(&d);
    }
}

fn mk_lib(domains: &Vec<ExtractedDomain>) {
    let f = File::create("cdp/src/lib.rs").unwrap();
    let mut buf = BufWriter::new(f);
    for d in domains.iter() {
        buf.write(format!("pub mod {};\n", case::to_snake_case(&d.domain)).as_bytes()).unwrap();
    }
    buf.flush();
}

fn mk_domain(domain: &ExtractedDomain) {
    let domain_path = format!("cdp/src/{}.rs", case::to_snake_case(&domain.domain));
    let f = File::create(domain_path).unwrap();
    let mut buf = BufWriter::new(f);
    println!("----------------------------------------------------------");
    println!("Domain Name: {:#?}", domain.domain);
    // println!("Domain Description: {:#?}", domain.description);
    match domain.description {
        None => (),
        Some(ref body) => mk_description(&mut buf, body),
    }
    buf.write("\n".as_bytes()).unwrap();
    buf.write("use std::str;\n".as_bytes()).unwrap();
    buf.write("\n".as_bytes()).unwrap();
    // println!("Dependencies: {:#?}", domain.dependencies);
    match domain.dependencies {
        None => (),
        Some(ref deps) => mk_dependencies(&mut buf, deps),
    }
    // println!("Deprecated: {:#?}", domain.deprecated);
    // match domain.deprecated {
    // None => (),
    // Some(deprec) => mk_deprecated(&mut buf, deprec),
    // }
    println!("Experimental: {:#?}", domain.experimental);
    // match domain.experimental {
    // None => (),
    // Some(experimental) => mk_experimental(&mut buf, experimental),
    // }
    //
    // println!("Types: {:#?}", domain.deprecated);
    match domain.extracted_types {
        None => (),
        Some(ref extracted_types) => mk_types(&mut buf, extracted_types),
    }
    // println!("Commands: {:#?}", domain.commands);
    match domain.commands {
        None => (),
        Some(ref commands) => mk_commands(&mut buf, commands),
    }
    // println!("Events: {:#?}", domain.events);
    match domain.events {
        None => (),
        Some(ref events) => mk_events(&mut buf, events),
    }
    println!("----------------------------------------------------------");
    buf.flush();
}

fn mk_description(buf: &mut BufWriter<File>, desc: &str) {
    buf.write(format!("/// {}\n", desc).as_bytes()).unwrap();
}

fn mk_dependencies(buf: &mut BufWriter<File>, deps: &Vec<String>) {
    for dep in deps.iter() {
        buf.write(format!("use {};\n", case::to_snake_case(dep)).as_bytes()).unwrap();
    }
}

fn mk_deprecated(buf: &mut BufWriter<File>, deprec: bool) {
    if deprec {
        buf.write(format!("#[deprecated(note = \"consult the Chrome DevTools Protocol viewer \
                            for more details.\")]\n")
                .as_bytes())
            .unwrap();
    }
}

fn mk_experimental(buf: &mut BufWriter<File>, experimental: bool) {
    if experimental {
        buf.write(format!("#![unstable()]\n").as_bytes())
            .unwrap();
    }
}

fn mk_types(buf: &mut BufWriter<File>, ts: &Vec<ExtractedType>) {}

fn mk_commands(buf: &mut BufWriter<File>, cmds: &Vec<ExtractedCommand>) {}

fn mk_events(buf: &mut BufWriter<File>, events: &Vec<ExtractedEvent>) {}
