// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

use remote_debugger::dom::BackendNodeId;
use remote_debugger::dom::NodeId;
use remote_debugger::dom::PseudoType;
use remote_debugger::dom::Rect;
use remote_debugger::page::FrameId;

struct CSS;

impl CSS {
    pub fn enable() {
        unimplemented!()
    }

    pub fn disable() {
        unimplemented!()
    }

    pub fn get_matched_styles_for_node(node_id: NodeId) -> io::Result<RequestedStyles> {
        unimplemented!()
    }

    pub fn get_inline_styles_for_node(node_id: NodeId)
                                      -> io::Result<(Option<CSSStyle>, Option<CSSStyle>)> {
        unimplemented!()
    }

    pub fn get_computed_style_for_node(node_id: NodeId)
                                       -> io::Result<Vec<CSSComputedStyleProperty>> {
        unimplemented!()
    }

    pub fn get_platform_fonts_for_node(node_id: NodeId) -> io::Result<Vec<PlatformFontUsage>> {
        unimplemented!()
    }

    pub fn get_style_sheet_text<'a>(style_sheet_id: StyleSheetId) -> io::Result<&'a str> {
        unimplemented!()
    }

    pub fn collect_class_names<'a>(style_sheet_id: StyleSheetId) -> io::Result<Vec<&'a str>> {
        unimplemented!()
    }

    pub fn set_style_sheet_text<'a>(style_sheet_id: StyleSheetId,
                                    text: &'a str)
                                    -> io::Result<Option<&'a str>> {
        unimplemented!()
    }

    pub fn set_rule_selector<'a>(style_sheet_id: StyleSheetId,
                                 range: SourceRange,
                                 selector: &'a str)
                                 -> io::Result<SelectorList> {
        unimplemented!()
    }

    pub fn set_keyframe_key<'a>(style_sheet_id: StyleSheetId,
                                range: SourceRange,
                                key_text: &'a str)
                                -> io::Result<Value> {
        unimplemented!()
    }

    pub fn set_style_texts(edits: Vec<StyleDeclarationEdit>) -> io::Result<Vec<CSSStyle>> {
        unimplemented!()
    }

    pub fn set_media_text<'a>(style_sheet_id: StyleSheetId,
                              range: SourceRange,
                              text: &'a str)
                              -> io::Result<CSSMedia> {
        unimplemented!()
    }

    pub fn create_style_sheet(frame_id: FrameId) -> io::Result<StyleSheetId> {
        unimplemented!()
    }

    pub fn add_rule(style_sheet_id: StyleSheetId,
                    rule_text: String,
                    location: SourceRange)
                    -> io::Result<CSSRule> {
        unimplemented!()
    }

    pub fn force_pseudo_state<'a>(node_id: NodeId, forced_pseudo_classes: Vec<&'a str>) {
        unimplemented!()
    }

    pub fn set_media_queries() -> io::Result<Vec<CSSMedia>> {
        unimplemented!()
    }

    pub fn set_effective_property_value_for_node(node_Id: NodeId,
                                                 property_name: &str,
                                                 value: &str) {
        unimplemented!()
    }

    pub fn get_background_colors<'a>(node_id: NodeId) -> io::Result<Option<Vec<&'a str>>> {
        unimplemented!()
    }

    pub fn start_rule_usage_tracking() {
        unimplemented!()
    }

    pub fn take_coverage_delta() -> io::Result<Vec<RuleUsage>> {
        unimplemented!()
    }

    pub fn stop_rule_usage_tracking() -> io::Result<Vec<RuleUsage>> {
        unimplemented!()
    }
}

struct RequestedStyles {
    inline_style: Option<CSSStyle>,
    attributes_style: Option<CSSStyle>,
    matched_css_rules: Option<Vec<RuleMatch>>,
    pseudo_elements: Option<Vec<PseudoElementMatches>>,
    inherited: Option<Vec<InheritedStyleEntry>>,
    css_keyframe_rules: Option<Vec<CSSKeyframesRule>>,
}

enum CSSEvent {
    MediaQueryResultChanged,
    FontsUpdated,
    StyleSheetChanged(StyleSheetId),
    StyleSheetAdded(CSSStyleSheetHeader),
    StyleSheetRemoved(StyleSheetId),
}

struct StyleSheetId(String);

enum StyleSheetOrigin {
    Injected,
    UserAgent,
    Inspector,
    Regular,
}

struct PseudoElementMatches {
    pseudo_type: PseudoType,
    matches: Vec<RuleMatch>,
}

struct InheritedStyleEntry {
    inline_style: Option<CSSStyle>,
    matched_css_rules: Vec<RuleMatch>,
}

struct RuleMatch {
    rule: CSSRule,
    matching_selectors: Vec<i32>,
}

struct Value {
    text: String,
    range: Option<SourceRange>,
}

struct SelectorList {
    selectors: Vec<Value>,
    text: String,
}

struct CSSStyleSheetHeader {
    style_sheet_id: StyleSheetId,
    frame_id: FrameId,
    source_url: String,
    source_map_url: Option<String>,
    origin: StyleSheetOrigin,
    title: String,
    owner_node: Option<BackendNodeId>,
    disabled: bool,
    has_source_url: bool,
    is_inline: bool,
    start_line: u32,
    start_column: u32,
    length: u32,
}

struct CSSRule {
    style_sheet_id: Option<StyleSheetId>,
    selector_list: SelectorList,
    origin: StyleSheetOrigin,
    style: CSSStyle,
    media: Option<Vec<CSSMedia>>,
}

struct RuleUsage {
    style_sheet_id: StyleSheetId,
    start_offset: u32,
    end_offset: u32,
    used: bool,
}

struct SourceRange {
    start_line: i32,
    start_column: i32,
    end_line: i32,
    end_column: i32,
}

struct ShorthandEntry {
    name: String,
    value: String,
    important: Option<bool>,
}

struct CSSComputedStyleProperty {
    name: String,
    value: String,
}

struct CSSStyle {
    style_sheet_id: Option<StyleSheetId>,
    css_properties: Vec<CSSProperty>,
    shorthand_entries: Vec<ShorthandEntry>,
    css_text: Option<String>,
    range: Option<SourceRange>,
}

struct CSSProperty {
    name: String,
    value: String,
    important: Option<bool>,
    implicit: Option<bool>,
    text: Option<String>,
    parsed_ok: Option<bool>,
    disabled: Option<bool>,
    range: Option<SourceRange>,
}

struct CSSMedia {
    text: String,
    source: String,
    source_url: Option<String>,
    range: Option<SourceRange>,
    style_sheet_id: Option<StyleSheetId>,
    media_list: Option<Vec<MediaQuery>>,
}

struct MediaQuery {
    expressions: Vec<MediaQueryExpression>,
    active: bool,
}

struct MediaQueryExpression {
    value: u32,
    unit: String,
    feature: String,
    value_range: Option<SourceRange>,
    computed_length: Option<u32>,
}

struct PlatformFontUsage {
    family_name: String,
    is_custom_font: bool,
    glyph_count: u32,
}

struct CSSKeyframesRule {
    animation_name: Value,
    keyframes: Vec<CSSKeyframesRule>,
}

struct CSSKeyframeRule {
    styleSheetId: Option<StyleSheetId>,
    origin: StyleSheetOrigin,
    key_text: Value,
    style: CSSStyle,
}

struct StyleDeclarationEdit {
    style_sheet_id: StyleSheetId,
    range: SourceRange,
    text: String,
}

pub struct InlineTextBox {
    bounding_box: Rect,
    start_character_index: i32,
    num_characters: i32,
}
