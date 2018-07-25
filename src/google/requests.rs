

#[derive(Serialize)]
pub struct RecognizeRequest {
    pub config: RecognitionConfig,
    pub audio: RecognitionAudio
}

#[derive(Serialize)]
pub enum AudioEncoding {
    #[serde(rename = "ENCODING_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "LINEAR16")]
    Linear16,
    #[serde(rename = "FLAC")]
    Flac,
    #[serde(rename = "MULAW")]
    Mulaw,
    #[serde(rename = "AMR")]
    Amr,
    #[serde(rename = "AMR_WB")]
    Amwwb,
    #[serde(rename = "OGG_OPUS")]
    OggOpus,
    #[serde(rename = "SPEEX_WITH_HEADER_BYTE")]
    SpeexWithHeaderByte

}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecognitionConfig {
    pub encoding: AudioEncoding,
    pub sample_rate_hertz: u32,
    pub language_code: String,
}

#[derive(Serialize)]
pub struct RecognitionAudio {
    pub content: String
}