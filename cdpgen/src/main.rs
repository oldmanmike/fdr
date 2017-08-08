// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use reqwest::Client;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct ExtractedCDP {
    version: ExtractedCDPVersion,
    domains: Vec<ExtractedDomain>,
}

#[derive(Debug, Deserialize)]
struct ExtractedCDPVersion {
    major: String,
    minor: String,
}

#[derive(Debug, Deserialize)]
struct ExtractedDomain {
    domain: String,
    description: Option<String>,
    dependencies: Option<Vec<String>>,
    deprecated: Option<bool>,
    #[serde(rename = "types")]
    extracted_types: Option<Vec<ExtractedType>>,
    commands: Option<Vec<ExtractedCommand>>,
    events: Option<Vec<ExtractedEvent>>,
}

#[derive(Debug, Deserialize)]
struct ExtractedType {
    id: String,
    #[serde(rename = "type")]
    extracted_type: String,
    description: Option<String>,
    properties: Option<Vec<ExtractedStructField>>,
    experimental: Option<bool>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct ExtractedCommand {
    name: String,
    parameters: Option<Vec<ExtractedCmdField>>,
    returns: Option<Vec<ExtractedReturnType>>,
    description: Option<String>,
    experimental: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct ExtractedCmdField {
    name: String,
    extracted_type: Option<String>,
    extracted_ref: Option<String>,
    optional: Option<bool>,
    experimental: Option<bool>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExtractedReturnType {
    name: String,
    extracted_type: Option<String>,
    extracted_ref: Option<String>,
    description: Option<String>,
    optional: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct ExtractedEvent {
    name: String,
    parameters: Option<Vec<ExtractedEventField>>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
enum EventItem {
    Singular(ExtractedEventFieldItem),
    Plural(Vec<ExtractedEventFieldItem>),
}

#[derive(Debug, Deserialize)]
struct ExtractedEventFieldItem {
    #[serde(rename = "type")]
    extracted_type: Option<String>,
    #[serde(rename = "$ref")]
    extracted_ref: Option<String>,
}

fn main() {
    println!("Generating Chrome Devtools Protocol bindings...");

    let mut resp1 = reqwest::get("http://raw.githubusercontent.\
                                 com/ChromeDevTools/devtools-protocol/master/json/js_protocol.\
                                 json")
        .unwrap();
    assert!(resp1.status().is_success());
    println!("js_protocol has arrived!");
    let mut js_content = String::new();
    resp1.read_to_string(&mut js_content);
    let js_proto: ExtractedCDP = serde_json::from_str(&js_content).unwrap();
    // println!("Got: {:?}", js_proto);

    let mut resp2 = reqwest::get("http://raw.githubusercontent.\
                      com/ChromeDevTools/devtools-protocol/master/json/browser_protocol.json")
        .unwrap();
    assert!(resp2.status().is_success());
    println!("browser_protocol has arrived!");
    let mut browser_content = String::new();
    resp2.read_to_string(&mut browser_content);
    let browser_proto: ExtractedCDP = serde_json::from_str(&browser_content).unwrap();
    // println!("Got: {:?}", browser_proto);

}
