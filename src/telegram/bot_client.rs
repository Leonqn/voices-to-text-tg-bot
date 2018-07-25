use hyper::Client;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use hyper::Body;
use hyper::rt::Future;
use serde::Serialize;
use serde::de::DeserializeOwned;
use hyper::Request;
use hyper::Uri;
use serde_json;
use hyper::rt::Stream;
use std::sync::Arc;
use telegram::requests::*;
use telegram::responses::*;
use error::Error;
use futures::stream;
use futures::future;

const BASE_URI: &'static str = "https://api.telegram.org/bot";

pub struct BotClient {
    http_client: Arc<Client<HttpsConnector<HttpConnector>, Body>>,
    token: String,
}

impl BotClient {
    pub fn new(http_client: Arc<Client<HttpsConnector<HttpConnector>, Body>>, token: String) -> BotClient {
        BotClient {
            http_client,
            token,
        }
    }

    pub fn incoming_voice_messages(client: Arc<BotClient>) -> impl Stream<Item=Message, Error=Error> {
        stream::unfold(None, move |offset| {
            let request =
                GetUpdates {
                    offset,
                    limit: None,
                    timeout: Some(20),
                    allowed_updates: vec!["message".to_string()],
                };
            let client = Arc::clone(&client);
            let map = client.get_updates(request).map(move |updates| {
                let (mut messages, mut offset) = (Vec::new(), None);
                for update in updates.into_iter() {
                    match update {
                        Update { update_id, message: message @ Some(Message {chat: Some(..), voice: Some(..), ..}) } => {
                            offset = Some(update_id);
                            messages.push(message.unwrap());
                        }
                        Update { update_id, .. } => {
                            offset = Some(update_id)
                        }
                    }
                }
                (messages, offset.map(|x| x + 1))
            });
            Some(map)
        })
            .filter(|x| !x.is_empty())
            .map(move |update| stream::futures_ordered(update.into_iter().map(|x| future::ok(x))))
            .flatten()
    }

    pub fn send_message(&self, request: SendMessage) -> impl Future<Item = Message, Error = Error> {
        self.send(request)
    }

    pub fn get_updates(&self, request: GetUpdates) -> impl Future<Item=Vec<Update>, Error=Error> {
        self.send(request)
    }

    pub fn get_file(&self, request: GetFile) -> impl Future<Item=Vec<u8>, Error=Error> {
        let token = self.token.clone();
        let http_client = Arc::clone(&self.http_client);
        self.send(request)
            .then(|file| {
                match file? {
                    File { file_path: Some(path), .. } => Ok(path),
                    _ => Err(Error::UnknownError("File not found".to_string()))
                }
            })
            .and_then(move |path| {
                let uri_to_file: Uri = format!("https://api.telegram.org/file/bot{}/{}", token, path).parse().unwrap();
                http_client
                    .get(uri_to_file)
                    .and_then(|response| {
                        response
                            .into_body()
                            .concat2()
                    })
                    .then(|x| Ok(x?.to_owned()))
            })
    }

    fn send<TRequest, TResult>(&self, request: TRequest) -> impl Future<Item=TResult, Error=Error>
        where TRequest: TgRequest + Serialize,
              TResult: DeserializeOwned
    {
        let uri = UriBuilder::new(&self.token).add_method(request.method());
        let request =
            Request::post(uri)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request).unwrap()))
                .expect("While creating request error has occurred");


        self.http_client.request(request)
            .and_then(|r| {
                let status = r.status();
                r.into_body().concat2().map(move |x| (status, x))
            })
            .then(|x| {
                let (status_code, body) = x?;
                let response: TgResponse<TResult> = serde_json::from_slice(&body)?;
                match response {
                    TgResponse { ok: true, result: Some(res), .. } => Ok(res),
                    TgResponse { ok: false, description: desc, .. } => Err(Error::Telegram(status_code, desc)),
                    _ => panic!("Should not happen")
                }
            })
    }
}

struct UriBuilder {
    uri: String
}

impl UriBuilder {
    fn new(token: &str) -> UriBuilder {
        let mut uri = String::from(BASE_URI);
        uri.push_str(token);
        uri.push_str("/");
        UriBuilder {
            uri
        }
    }

    fn add_method(&mut self, method: &str) -> Uri {
        self.uri.push_str(method);
        self.uri.parse().expect("Unexpected uri happens")
    }
}
