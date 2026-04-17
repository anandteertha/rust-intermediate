#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EventType {
    LoginSuccess,
    LoginFailure,
    PaymentProcessed,
    PaymentFailed,
    EmailSent,
    SmsSent,
}
