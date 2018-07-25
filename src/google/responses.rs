#[derive(Deserialize, Debug)]
pub struct SpeechRecognitionResponse {
    pub results: Vec<SpeechRecognitionResult>
}

#[derive(Deserialize, Debug)]
pub struct SpeechRecognitionResult {
    pub alternatives: Vec<SpeechRecognitionAlternative>
}

#[derive(Deserialize, Debug)]
pub struct SpeechRecognitionAlternative {
    pub transcript: String,
    pub confidence: f64,
}