use std::thread;

use crate::{event::Event, event_summary::EventSummary, event_type::EventType};
mod event;
mod event_summary;
mod event_type;
fn main() {
    let mut event_summary = EventSummary::new();

    // create the channel
    let (tx, rx) = std::sync::mpsc::channel::<Event>();

    // create worker threads for 3 producer and send the events.
    let notification_service_tx = tx.clone();
    let payment_service_tx = tx.clone();
    let login_service_tx = tx.clone();

    let notification_handle = thread::spawn(move || {
        notification_service_tx
            .send(Event::new(EventType::EmailSent, "Notification Service"))
            .unwrap();
        notification_service_tx
            .send(Event::new(EventType::SmsSent, "Notification Service"))
            .unwrap();
    });
    let payment_service_handle = thread::spawn(move || {
        payment_service_tx
            .send(Event::new(EventType::PaymentFailed, "Payment Service"))
            .unwrap();
        payment_service_tx
            .send(Event::new(EventType::PaymentProcessed, "Payment Service"))
            .unwrap();
    });
    let login_service_handle = thread::spawn(move || {
        login_service_tx
            .send(Event::new(EventType::LoginFailure, "Login Service"))
            .unwrap();
        login_service_tx
            .send(Event::new(EventType::LoginSuccess, "Login Service"))
            .unwrap();
    });

    // drop the original sender so it waits till all the workers have dropped and the program don't halts forever.
    drop(tx);

    // receiver will loop until senders/producers are available.
    while let Ok(event) = rx.recv() {
        event_summary.add_event(event);
    }

    notification_handle.join().unwrap();
    payment_service_handle.join().unwrap();
    login_service_handle.join().unwrap();

    event_summary.print_summary_stats();
}
