pub trait TgRequest {
    fn method(&self) -> &'static str;
}

impl TgRequest for GetUpdates {
    fn method(&self) -> &'static str {
        "getUpdates"
    }
}

impl TgRequest for GetFile {
    fn method(&self) -> &'static str {
        "getFile"
    }
}

impl TgRequest for SendMessage {
    fn method(&self) -> &'static str {
        "sendMessage"
    }
}

#[derive(Serialize, Debug)]
pub struct SendMessage {
    pub chat_id: i32,
    pub text: String,
    pub reply_to_message_id: Option<i32>
}

#[derive(Serialize, Debug)]
pub struct GetFile {
    pub file_id: String
}

#[derive(Serialize, Debug)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_updates: Vec<String>,
}