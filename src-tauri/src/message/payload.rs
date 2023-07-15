// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub base64_vector_scope: String,
    pub base64_waveform: String,
}

impl Payload {
    pub fn new(base64_vector_scope: String, base64_waveform: String) -> Self {
        Payload {
            base64_vector_scope,
            base64_waveform,
        }
    }
}
