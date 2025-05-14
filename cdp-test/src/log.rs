use crate::common::*;
use crate::network::*;
/// Log entry.
pub struct LogEntry {
    pub source: Box<String>,
    pub level: Box<String>,
    pub text: Box<String>,
    pub category: Box<String>,
    pub timestamp: Box<Timestamp>,
    pub url: Box<String>,
    pub line_number: Box<i64>,
    pub stack_trace: Box<StackTrace>,
    pub network_request_id: Box<NetworkRequestId>,
    pub worker_id: Box<String>,
    pub args: (),
}
/// Violation configuration setting.
pub struct ViolationSetting {
    pub name: Box<String>,
    pub threshold: Box<u64>,
}
