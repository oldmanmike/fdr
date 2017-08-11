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
use std::io;
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
    items: Option<ArrayItemType>,
    #[serde(rename = "minItems")]
    min_items: Option<usize>,
    #[serde(rename = "maxItems")]
    max_items: Option<usize>,
    description: Option<String>,
    #[serde(rename = "enum")]
    extracted_enum: Option<Vec<String>>,
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
    items: Option<ArrayItemType>,
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

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ArrayItemType {
    #[serde(rename = "$ref")]
    extracted_ref: Option<String>,
    #[serde(rename = "type")]
    extracted_type: Option<String>,
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
    match domain.commands {
        None => (),
        Some(ref commands) => mk_commands(&mut buf, commands),
    }
    match domain.events {
        None => (),
        Some(ref events) => mk_events(&mut buf, events),
    }
    println!("----------------------------------------------------------");
    buf.flush();
}

fn mk_description(buf: &mut BufWriter<File>, desc: &str) {
    put_description(buf, &sanitize_comment(desc)).unwrap();
}

fn mk_dependencies(buf: &mut BufWriter<File>, deps: &Vec<String>) {
    for dep in deps.iter() {
        buf.write(format!("use {};\n", case::to_snake_case(dep)).as_bytes()).unwrap();
    }
    buf.write(format!("\n").as_bytes()).unwrap();
}

fn mk_deprecated(buf: &mut BufWriter<File>, deprec: bool) {
    put_deprecated(buf, deprec).unwrap();
}

fn mk_experimental(buf: &mut BufWriter<File>, experimental: bool) {
    put_experimental(buf, experimental).unwrap();
}

fn mk_types(buf: &mut BufWriter<File>, ts: &Vec<ExtractedType>) {
    for t in ts {
        match t.description {
            None => (),
            Some(ref body) => put_description(buf, body).unwrap(),
        }
        match t.experimental {
            None => (),
            Some(experimental) => put_experimental(buf, experimental).unwrap(),
        }
        match t.properties {
            None => {
                match t.extracted_enum {
                    None => {
                        let fmt_struct_name = t.id;
                        let fmt_struct_type = convert_struct_type(&t.extracted_type).unwrap();
                        put_ln(buf,
                               format!("pub struct {}({});\n", fmt_struct_name, fmt_struct_type))
                    }
                    Some(ref enums) => {
                        buf.write_all(format!("pub enum {} {{\n", t.id).as_bytes()).unwrap();
                        for e in enums.iter() {
                            match e.as_ref() {
                                "-0" => put_ln(buf, format!("    {},\n", "NegZero")),
                                "-infinity" => put_ln(buf, format!("    {},\n", "NegInfinity")),
                                _ => {
                                    let fmt_name = case::to_pascal_case(e);
                                    put_ln(buf, format!("    {},\n", fmt_name))
                                }
                            }
                        }
                        buf.write_all(format!("}}\n").as_bytes()).unwrap();
                    }
                }
            }
            Some(ref fields) => {
                // println!("Printing fields...");
                put_ln(buf, format!("pub struct {} {{\n", t.id));
                for f in fields.iter() {
                    match f.clone().description {
                        None => (),
                        Some(desc) => {
                            let comment = sanitize_comment(&desc);
                            put_ln(buf, format!("    /// {}\n", comment))
                        }
                    }
                    let field_name = convert_name(&t.id, &f.name);
                    if let Some(ref ft) = f.extracted_type {
                        match convert_field_type(f, ft) {
                            None => println!("Failed to convert: {:#?}", f),
                            Some(converted) => {
                                let field_type = converted;
                                if let Some(true) = f.optional {
                                    put_ln(buf,
                                           format!("    pub {}: Option<{}>,\n",
                                                   field_name,
                                                   field_type));
                                } else {
                                    put_ln(buf,
                                           format!("    pub {}: {},\n", field_name, field_type));
                                }
                            }
                        }
                    }
                    if let Some(ref ft) = f.extracted_ref {
                        put_ln(buf, format!("    pub {}: {},\n", field_name, ft));
                    }
                }
                put_ln(buf, format!("}}\n"));
            }
        }
        put_ln(buf, format!("\n"));
    }
}

fn mk_commands(buf: &mut BufWriter<File>, cmds: &Vec<ExtractedCommand>) {}

fn mk_events(buf: &mut BufWriter<File>, events: &Vec<ExtractedEvent>) {}

fn put_description(buf: &mut BufWriter<File>, desc: &str) -> io::Result<()> {
    buf.write_all(format!("/// {}\n", desc).as_bytes())
}

fn put_deprecated(buf: &mut BufWriter<File>, enable: bool) -> io::Result<()> {
    if enable {
        buf.write(format!("#[deprecated(note = \"consult the Chrome DevTools Protocol viewer \
                            for more details.\")]\n")
            .as_bytes());
    }
    Ok(())
}

fn put_experimental(buf: &mut BufWriter<File>, enable: bool) -> io::Result<()> {
    if enable {
        buf.write_all(format!("#![unstable()]\n").as_bytes());
    }
    Ok(())
}

fn convert_struct_type(js_type: &str) -> Option<String> {
    println!("Got: {:#?}", js_type);
    match js_type {
        "string" => Some("String".to_string()),
        "integer" => Some("i32".to_string()),
        "number" => Some("u32".to_string()),
        "boolean" => Some("bool".to_string()),
        _ => None,
    }
}

fn convert_field_type(f: &ExtractedStructField, js_type: &str) -> Option<String> {
    // println!("Converting type {} from {:#?}...", js_type, f);
    match f.extracted_ref {
        None => {
            // Just a JS type, needs to be converted into Rust equivalent
            match js_type {
                "string" => Some("String".to_string()),
                "integer" => Some("i32".to_string()),
                "number" => Some("u32".to_string()),
                "boolean" => Some("bool".to_string()),
                "object" => {
                    let type_name = case::to_pascal_case(&f.name);
                    Some(type_name)
                }
                "array" => {
                    match f.clone().items {
                        None => None,
                        Some(item_type) => {
                            if let Some(custom_type) = item_type.extracted_ref {
                                let item = case::to_pascal_case(&custom_type);
                                Some(format!("Vec<{}>", item))
                            } else {
                                match item_type.extracted_type.unwrap().as_ref() {
                                    "string" => Some("Vec<String>".to_string()),
                                    "integer" => Some("Vec<i32>".to_string()),
                                    "number" => Some("Vec<u32>".to_string()),
                                    "boolean" => Some("Vec<bool>".to_string()),
                                    _ => None,
                                }
                            }
                        }
                    }
                }
                _ => None,
            }
        }
        Some(ref dependency_ref) => {
            // The type is an API type imported from another domain
            let chunks: Vec<&str> = dependency_ref.split('.').collect();
            println!("{:?}", chunks);
            if chunks.len() == 2 {
                let dep_str = format!("{}::{}", case::to_snake_case(chunks[0]), chunks[1]);
                println!("Formatted Dependency: {}", dep_str);
                Some(dep_str)
            } else {
                panic!("Splitting {} a the dot resulted in more than 2 parts!")
            }
        }
    }
}

fn convert_name(prefix: &str, type_name: &str) -> String {
    let sanitized_prefix = case::to_snake_case(prefix);
    let sanitized_type_name = case::to_snake_case(type_name);
    if sanitized_type_name == "type" {
        return format!("{}_{}", sanitized_prefix, sanitized_type_name);
    } else {
        return sanitized_type_name.to_string();
    }
}

fn sanitize_comment(comment: &str) -> String {
    let comment1 = comment.replace("<code>", "`");
    let comment2 = comment1.replace("</code>", "`");
    let comment3 = comment2.replace("<p>", "\n///\n");
    let comment4 = comment3.replace("</p>", "");
    comment4
}

fn put_ln(buf: &mut BufWriter<File>, line: String) {
    buf.write_all(line.as_bytes()).unwrap();
}
