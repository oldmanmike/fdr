use std::io;

use remote_debugger::dom::{NodeId, BackendNodeId};

struct AXNodeId(String);

enum AXValueType {
    Boolean,
    Tristate,
    BooleanOrUndefined,
    Idref,
    IdrefList,
    Integer,
    Node,
    NodeList,
    Number,
    String,
    ComputedString,
    Token,
    TokenList,
    DomRelation,
    Role,
    InternalRole,
    ValueUndefined,
}

enum AXValueSourceType {
    Attribute,
    Implicit,
    Style,
    Contents,
    Placeholder,
    RelatedElement,
}

enum AXValueNativeSourceType {
    Figcaption,
    Label,
    Labelfor,
    Labelwrapped,
    Legend,
    Tablecaption,
    Title,
    Other,
}

struct AXValueSource<A> {
    ax_type: AXValueSourceType,
    value: Option<AXValue<A>>,
    attribute: Option<String>,
    attribute_value: Option<String>,
    superseded: Option<bool>,
    native_source: Option<AXValueNativeSourceType>,
    native_source_value: Option<AXValue<A>>,
    invalid: Option<bool>,
    invalid_reason: Option<String>,
}

struct AXRelatedNode {
    backend_dom_node_id: BackendNodeId,
    idref: Option<String>,
    text: Option<String>,
}

struct AXProperty<A> {
    name: String,
    value: AXValue<A>,
}

struct AXValue<A> {
    ax_type: AXValueType,
    value: Option<A>,
    related_nodes: Option<Vec<AXRelatedNode>>,
    sources: Option<Vec<AXValueSource<A>>>,
}

enum AXGlobalStates {
    Disabled,
    Hidden,
    HiddenRoot,
    Invalid,
    Keyshortcuts,
    Roledescription,
}

enum AXLiveRegionAttributes {
    Live,
    Atomic,
    Relevant,
    Busy,
    Root,
}

enum AXWidgetAttributes {
    Autocomplete,
    Haspopup,
    Level,
    Multiselectable,
    Orientation,
    Multiline,
    Readonly,
    Required,
    Valuemin,
    Valuemax,
    Valuetext,
}

enum AXWidgetStates {
    Checked,
    Expanded,
    Modal,
    Pressed,
    Selected,
}

enum AXRelationshipAttributes {
    Activedescendant,
    Controls,
    Describedby,
    Details,
    Errormessage,
    Flowto,
    Labelledby,
    Owns,
}

pub struct AXNode<A> {
    node_id: AXNodeId,
    ignored: bool,
    ignored_reasons: Option<Vec<AXProperty<A>>>,
    ignored_role: Option<AXValue<A>>,
    ignored_name: Option<AXValue<A>>,
    description: Option<AXValue<A>>,
    value: Option<AXValue<A>>,
    properties: Option<Vec<AXProperty<A>>>,
    child_ids: Option<Vec<AXNodeId>>,
    backend_dom_node_id: Option<BackendNodeId>,
}

pub struct Accessibility;

impl Accessibility {
    pub fn get_partial_ax_tree<A>(node_id: NodeId,
                                  fetch_relatives: Option<bool>)
                                  -> io::Result<Vec<AXNode<A>>> {
        unimplemented!()
    }
}
