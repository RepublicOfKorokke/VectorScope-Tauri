// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: String,
}

impl Payload {
    pub fn new(message: String) -> Self {
        Payload { message: message }
    }
}
