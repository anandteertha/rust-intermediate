use crate::event_type::EventType;

pub struct Event {
    event_type: EventType,
    producer: String,
}

impl Event {
    pub fn new(event_type: EventType, producer: &str) -> Self {
        Self {
            event_type,
            producer: producer.to_owned(),
        }
    }
    pub fn get_event_type(&self) -> EventType {
        self.event_type
    }
    pub fn get_producer(&self) -> String {
        self.producer.to_owned()
    }
}
