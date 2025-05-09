use crate::network::*;
/// Log entry.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Log/#type-LogEntry>
pub struct LogEntry {
    pub source: String,
    pub level: String,
    pub text: String,
    pub category: String,
    pub timestamp: (),
    pub url: String,
    pub line_number: i64,
    pub stack_trace: (),
    pub network_request_id: (),
    pub worker_id: String,
    pub args: (),
}
/// Violation configuration setting.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Log/#type-ViolationSetting>
pub struct LogViolationSetting {
    pub name: String,
    pub threshold: u64,
}
