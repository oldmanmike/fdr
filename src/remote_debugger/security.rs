use std::io;

pub struct CertificateId(i32);

pub enum MixedContentType {
    Blockable,
    OptionallyBlockable,
    None,
}

pub enum SecurityState {
    Unknown,
    Neutral,
    Insecure,
    Warning,
    Secure,
    Info,
}

struct SecurityStateExplanation {
    security_state: SecurityState,
    summary: String,
    description: String,
    has_certificate: bool,
    mixed_content_type: MixedContentType,
}

struct InsecureContentStatus {
    ran_mixed_content: bool,
    displayed_mixed_content: bool,
    contained_mixed_form: bool,
    ran_content_with_cert_errors: bool,
    displayed_content_with_cert_errors: bool,
    ran_insecure_content_style: SecurityState,
    displayed_insecure_content_style: SecurityState,
}

enum CertificateErrorAction {
    Continue,
    Cancel,
}

enum SecurityEvent {
    SecurityStateChanged {
        security_state: SecurityState,
        scheme_is_crytographic: bool,
        explanations: Vec<SecurityStateExplanation>,
        insecure_content_status: InsecureContentStatus,
        summary: Option<String>,
    },
    CertificateError {
        event_id: i32,
        error_type: String,
        request_url: String,
    },
}

struct Security;

impl Security {
    fn enable() {
        unimplemented!()
    }
    fn disable() {
        unimplemented!()
    }
    fn show_certificate_viewer() {
        unimplemented!()
    }
    fn handle_certificate_error(event_id: i32, action: CertificateErrorAction) {
        unimplemented!()
    }
    fn set_override_certificate_errors(override_cert_error: bool) {
        unimplemented!()
    }
}
