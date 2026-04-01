use crate::event_severity::Severity;

pub struct Event {
    service: String,
    severity: Severity,
}
