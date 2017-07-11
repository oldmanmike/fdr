use std::io;

use remote_debugger::page::FrameId;
use remote_debugger::page::ResourceType;
use remote_debugger::runtime::StackTrace;
use remote_debugger::security::MixedContentType;
use remote_debugger::security::SecurityState;

pub struct LoaderId(String);

pub struct RequestId(String);

struct InterceptionId(String);

enum ErrorReason {
    Failed,
    Aborted,
    TimedOut,
    AccessDenied,
    ConnectionClosed,
    ConnectionReset,
    ConnectionRefused,
    ConnectionAborted,
    ConnectionFailed,
    NameNotResolved,
    InternetDisconnected,
    AddressUnreachable,
}

pub struct TimeSinceEpoch(u64);

struct MonotonicTime(u64);

struct Headers;

enum ConnectionType {
    None,
    Cellural2G,
    Cellural3G,
    Cellural4G,
    Bluetooth,
    Ethernet,
    Wifi,
    Wimax,
    Other,
}

enum CookieSameSite {
    Strict,
    Lax,
}

struct ResourceTiming {
    request_time: u32,
    proxy_start: u32,
    proxy_end: u32,
    dns_start: u32,
    dns_end: u32,
    connect_start: u32,
    connect_end: u32,
    ssl_start: u32,
    ssl_end: u32,
    worker_start: u32,
    worker_ready: u32,
    send_start: u32,
    send_end: u32,
    push_start: u32,
    push_end: u32,
    receive_headers_end: u32,
}

enum ResourcePriority {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

struct Request {
    url: String,
    method: String,
    headers: Headers,
    post_data: Option<String>,
    mixed_content_type: Option<MixedContentType>,
    initial_priority: ResourcePriority,
    referrer_policy: String,
    is_link_preload: Option<bool>,
}

struct SignedCertificateTimestamp {
    status: String,
    origin: String,
    log_description: String,
    log_id: String,
    timestamp: TimeSinceEpoch,
    hash_algorithm: String,
    signature_algorithm: String,
    signature_data: String,
}

struct SecurityDetails {
    protocol: String,
    key_exchange: String,
    key_exchange_group: Option<String>,
    cipher: String,
    subject_name: String,
    san_list: Vec<String>,
    issuer: String,
    valid_form: TimeSinceEpoch,
    valid_to: TimeSinceEpoch,
    signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp>,
}

enum BlockedReason {
    CSP,
    MixedContent,
    Origin,
    Inspector,
    SubresourceFilter,
    Other,
}

struct Response {
    url: String,
    status: u32,
    status_text: String,
    headers: Headers,
    headers_text: Option<String>,
    mime_type: String,
    request_headers: Option<Headers>,
    request_headers_text: Option<String>,
    connection_reused: bool,
    connection_id: u32,
    remote_ip_address: Option<String>,
    remote_port: Option<i32>,
    from_disk_cache: Option<bool>,
    from_service_worker: Option<bool>,
    encoded_data_length: u32,
    timing: Option<ResourceTiming>,
    protocol: Option<String>,
    security_state: SecurityState,
    security_details: Option<SecurityDetails>,
}

struct WebSocketRequest {
    headers: Headers,
}

struct WebSocketResponse {
    status: u32,
    status_text: String,
    headers: Headers,
    headers_text: Option<String>,
    request_headers: Option<Headers>,
    request_headers_text: Option<String>,
}

struct WebSocketFrame {
    opcode: u32,
    mask: bool,
    payload_data: String,
}

struct CachedResource {
    url: String,
    resource_type: ResourceType,
    response: Option<Response>,
    body_size: u32,
}

struct Initiator {
    initiator_type: String,
    stack: Option<StackTrace>,
    url: Option<String>,
    line_number: Option<u32>,
}

struct Cookie {
    name: String,
    value: String,
    domain: String,
    path: String,
    expires: u32,
    size: i32,
    http_only: bool,
    secure: bool,
    session: bool,
    same_site: Option<CookieSameSite>,
}

struct AuthChallenge {
    source: Option<String>,
    origin: String,
    scheme: String,
    realm: String,
}

struct AuthChallengeResponse {
    response: String,
    username: Option<String>,
    password: Option<String>,
}

enum NetworkEvent {
    ResourceChangedPriority {
        request_id: RequestId,
        new_priority: ResourcePriority,
        timestamp: MonotonicTime,
    },
    RequestWillBeSent {
        request_id: RequestId,
        loader_id: LoaderId,
        document_url: String,
        request: Request,
        timestamp: MonotonicTime,
        wall_time: TimeSinceEpoch,
        initiator: Initiator,
        redirect_response: Option<Response>,
        resource_type: Option<ResourceType>,
        frame_id: Option<FrameId>,
    },
    RequestServedFromCache { request_id: RequestId },
    ResponseReceived {
        request_id: RequestId,
        loader_id: LoaderId,
        timestamp: MonotonicTime,
        resource_type: ResourceType,
        response: Response,
        frame_id: Option<FrameId>,
    },
    DataReceived {
        request_id: RequestId,
        timestamp: MonotonicTime,
        data_length: i32,
        encoded_data_length: i32,
    },
    LoadingFinished {
        request_id: RequestId,
        timestamp: MonotonicTime,
        encoded_data_length: u32,
    },
    LoadingFailed {
        request_id: RequestId,
        timestamp: MonotonicTime,
        resource_type: ResourceType,
        error_text: String,
        canceled: Option<bool>,
        blocked_reason: Option<BlockedReason>,
    },
    WebSocketWillSendHandshakeRequest {
        request_id: RequestId,
        timestamp: MonotonicTime,
        wall_time: TimeSinceEpoch,
        request: WebSocketRequest,
    },
    WebSockethandshakeResponseReceived {
        request_id: RequestId,
        timestamp: MonotonicTime,
        response: WebSocketResponse,
    },
    WebSocketCreated {
        request_id: RequestId,
        url: String,
        initiator: Option<Initiator>,
    },
    WebSocketClosed {
        request_id: RequestId,
        timestamp: MonotonicTime,
    },
    WebSocketFrameReceived {
        request_id: RequestId,
        timestamp: MonotonicTime,
        response: WebSocketFrame,
    },
    WebSocketFrameError {
        request_id: RequestId,
        timestamp: MonotonicTime,
        errorMessage: String,
    },
    EventSourceMessageReceived {
        request_id: RequestId,
        timestamp: MonotonicTime,
        event_name: String,
        event_id: String,
        data: String,
    },
    RequestIntercepted {
        interception_id: InterceptionId,
        request: Request,
        resource_type: ResourceType,
        redirect_headers: Option<Headers>,
        redirect_status_code: Option<i32>,
        redirect_url: Option<String>,
        auth_challenge: Option<AuthChallenge>,
    },
}

struct Network;

impl Network {
    fn enable(max_total_buffer_size: Option<i32>, max_resource_buffer_size: Option<i32>) {
        unimplemented!()
    }
    fn disable() {
        unimplemented!()
    }

    fn set_user_agent_override(user_agent: String) {
        unimplemented!()
    }

    fn set_extra_http_headers(headers: Headers) {
        unimplemented!()
    }

    fn get_response_body<'a>(request_id: RequestId) -> io::Result<(&'a str, bool)> {
        unimplemented!()
    }

    fn set_blocked_urls(url: Vec<&str>) {
        unimplemented!()
    }

    fn replay_xhr(request_id: RequestId) {
        unimplemented!()
    }

    fn can_clear_browser_cache() -> io::Result<bool> {
        unimplemented!()
    }

    fn clear_browser_cache() {
        unimplemented!()
    }

    fn get_cookies(urls: Option<Vec<&str>>) -> io::Result<Vec<Cookie>> {
        unimplemented!()
    }

    fn get_all_cookies() -> io::Result<Vec<Cookie>> {
        unimplemented!()
    }

    fn delete_cookie(cookie_name: &str, url: &str) {
        unimplemented!()
    }

    fn set_cookie(url: &str,
                  name: &str,
                  value: &str,
                  domain: Option<&str>,
                  path: Option<&str>,
                  secure: Option<bool>,
                  http_only: Option<bool>,
                  same_site: Option<CookieSameSite>,
                  expiration_date: Option<TimeSinceEpoch>)
                  -> io::Result<bool> {
        unimplemented!()
    }

    fn can_emulate_network_conditions() -> io::Result<bool> {
        unimplemented!()
    }

    fn emulate_network_conditions(offline: bool,
                                  latency: u32,
                                  download_throughput: u32,
                                  upload_throughput: u32,
                                  connection_type: Option<ConnectionType>) {
        unimplemented!()
    }

    fn set_cache_disabled(cache_disabled: bool) {
        unimplemented!()
    }

    fn set_bypass_service_worker(bypass: bool) {
        unimplemented!()
    }

    fn set_data_size_limits_for_test(max_total_size: i32, max_resource_size: i32) {
        unimplemented!()
    }

    fn get_certificate<'a>(origin: &str) -> io::Result<Vec<&'a str>> {
        unimplemented!()
    }

    fn set_request_interception_enabled(enabled: bool) {
        unimplemented!()
    }

    fn continue_intercepted_request(interception_id: InterceptionId,
                                    error_reason: Option<ErrorReason>,
                                    raw_response: Option<&str>,
                                    url: Option<&str>,
                                    method: Option<&str>,
                                    post_data: Option<&str>,
                                    headers: Option<Headers>,
                                    auth_challenge_reponse: Option<AuthChallengeResponse>) {
        unimplemented!()
    }
}
