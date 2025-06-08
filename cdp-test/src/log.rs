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
/// Clears the log.
pub type LogClear = ();
/// Disables log domain, prevents further log entries from being reported to the client.
pub type LogDisable = ();
/** Enables log domain, sends the entries collected so far to the client by means of the
`entryAdded` notification.*/
pub type LogEnable = ();
/// start violation reporting.
pub type LogStartViolationsReport = ();
/// Stop violation reporting.
pub type LogStopViolationsReport = ();
