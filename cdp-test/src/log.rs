use crate::network::*;
/// Log entry.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Log/#type-LogEntry>
pub struct LogEntry {
    pub source: (),
    pub level: (),
    pub text: (),
    pub category: (),
    pub timestamp: (),
    pub url: (),
    pub line_number: (),
    pub stack_trace: (),
    pub network_request_id: (),
    pub worker_id: (),
    pub args: (),
}
/// Violation configuration setting.
/// <https://chromedevtools.github.io/devtools-protocol/tot/Log/#type-ViolationSetting>
pub struct LogViolationSetting {
    pub name: (),
    pub threshold: (),
}
