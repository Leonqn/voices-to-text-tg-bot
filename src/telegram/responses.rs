#[derive(Deserialize, Debug)]
pub struct TgResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: i32,
    pub message: Option<Message>
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: i32,
    pub chat: Option<Chat>,
    pub user: Option<User>,
    pub voice: Option<Voice>

}

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: i32
}
#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Voice {
    pub file_id: String,
    pub duration: i32,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>
}

#[derive(Deserialize, Debug)]
pub struct File {
    pub file_id: String,
    pub file_size: Option<i32>,
    pub file_path: Option<String>

}