use hyper::Client;
use std::sync::Arc;
use google::requests::RecognizeRequest;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use hyper::Body;
use hyper::Request;
use serde_json;
use hyper::rt::Future;
use hyper::rt::Stream;
use google::responses::SpeechRecognitionResponse;
use hyper::Uri;
use google::error::Error;

pub struct SpeechClient {
    http_client: Arc<Client<HttpsConnector<HttpConnector>, Body>>,
    api_key: String,
}

impl SpeechClient {
    pub fn new(client: Arc<Client<HttpsConnector<HttpConnector>, Body>>, api_key: String) -> SpeechClient {
        SpeechClient {
            http_client: client,
            api_key,
        }
    }

    pub fn recognize(&self, request: RecognizeRequest) -> impl Future<Item = SpeechRecognitionResponse, Error = Error> {
        let uri: Uri = format!("https://speech.googleapis.com/v1/speech:recognize?key={}", self.api_key).parse().expect("Wrong google api uri format. Probably token contains illegal chars");
        let request =
            Request::post(uri)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request).unwrap()))
                .expect("While creating request an error has occurred");

        self.http_client.request(request)
            .and_then(|r| r.into_body().concat2())
            .then(|x| {
                Ok(serde_json::from_slice(&x?)?)
            })
    }
}