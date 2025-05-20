use crate::common::*;
use crate::network::*;
/// Log entry.
pub struct LogEntry {
    pub source: String,
    pub level: String,
    pub text: String,
    pub category: String,
    pub timestamp: Box<()>,
    pub url: String,
    pub line_number: i64,
    pub stack_trace: Box<()>,
    pub network_request_id: Box<NetworkRequestId>,
    pub worker_id: String,
    pub args: Vec<()>,
}
/// Violation configuration setting.
pub struct ViolationSetting {
    pub name: String,
    pub threshold: u64,
}
pub type LogClear = ();
pub type LogDisable = ();
pub type LogEnable = ();
pub type LogStartViolationsReport = ();
pub type LogStopViolationsReport = ();
