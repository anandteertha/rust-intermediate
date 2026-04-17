use std::collections::HashMap;

use crate::{event::Event, event_type::EventType};

pub struct EventSummary {
    total_events: usize,
    total_success: usize,
    total_failures: usize,
    per_producer: HashMap<String, usize>,
    per_event_type: HashMap<EventType, usize>,
}

impl EventSummary {
    pub fn new() -> Self {
        Self {
            total_events: 0,
            total_success: 0,
            total_failures: 0,
            per_producer: HashMap::new(),
            per_event_type: HashMap::new(),
        }
    }
    pub fn add_event(&mut self, event: Event) {
        self.total_events += 1;
        // update the per event type count
        self.per_event_type
            .entry(event.get_event_type())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        self.per_producer
            .entry(event.get_producer())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        if matches!(event.get_event_type(), EventType::LoginFailure)
            || matches!(event.get_event_type(), EventType::PaymentFailed)
        {
            self.total_failures += 1;
        } else if matches!(event.get_event_type(), EventType::LoginSuccess)
            || matches!(event.get_event_type(), EventType::PaymentProcessed)
        {
            self.total_success += 1;
        }
    }

    pub fn print_summary_stats(&self) {
        println!(
            "total events: {}\ntotal failures: {}\ntotal success:{}\nper producer: {:?}\nper event: {:?}",
            self.total_events,
            self.total_failures,
            self.total_success,
            self.per_producer,
            self.per_event_type
        )
    }
}
