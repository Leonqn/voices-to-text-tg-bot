extern crate voices_to_text;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate hyper_tls;
extern crate base64;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate config;
extern crate relegram;

use hyper::rt::Future;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::sync::Arc;
use hyper::Body;
use base64::encode;
use hyper::rt::Stream;
use config::Config;
use relegram::requests::*;
use relegram::responses::*;
use relegram::{HttpClient, BotApiClient};
use std::time::Duration;
use voices_to_text::google::*;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub bot_apikey: String,
    pub speech_apikey: String,
    pub lang: String,
}

fn main() {
    pretty_env_logger::init();
    info!("Started");

    let mut settings = Config::default();
    settings
        .merge(config::Environment::new())
        .expect("can't find settings");

    let Settings { bot_apikey, speech_apikey, lang }: Settings = settings.try_into().expect("Wrong settings");

    let https = HttpsConnector::new(1).expect("TLS initialization failed");
    let http_client = Arc::new(Client::builder().build::<_, Body>(https));
    let bot_client = Arc::new(BotApiClient::new(HttpClient::Arc(Arc::clone(&http_client)), bot_apikey));
    let speech_client = Arc::new(SpeechClient::new(Arc::clone(&http_client), speech_apikey));
    let default_timeout = Duration::from_secs(10);
    let get_updates = GetUpdatesRequest {
        offset: None,
        limit: None,
        timeout: Some(30),
        allowed_updates: None,
    };
    let fut =
        bot_client.incoming_updates(get_updates)
            .then(Ok)
            .for_each(move |update| {
                match update {
                    Ok(update) => {
                        match update.kind {
                            UpdateKind::Message(Message { id, from: MessageFrom::User { chat, .. }, kind: MessageKind::Voice { voice, .. }, .. }) => {
                                let speech_client = Arc::clone(&speech_client);
                                let lang = lang.clone();
                                let bot_client = Arc::clone(&bot_client);
                                let send_recognized_fut =
                                    bot_client
                                        .download_file(&GetFileRequest { file_id: voice.file_id }, default_timeout)
                                        .map_err(|x| error!("Error occurred while downloading voice {:?}", x))
                                        .and_then(|voice| recognize(speech_client, voice, lang))
                                        .and_then(move |recognized|
                                            bot_client
                                                .send_message(&SendMessageRequest {
                                                    chat_id: ChatId::Id(chat.id),
                                                    kind: SendMessageKind::Text(SendText::new(recognized)),
                                                    disable_notification: false,
                                                    reply_to_message_id: Some(id),
                                                    reply_markup: None,
                                                }, default_timeout)
                                                .map_err(|x| error!("Error occurred while sending recognized response {:?}", x)))
                                        .map(|_| ());
                                hyper::rt::spawn(send_recognized_fut);
                                Ok(())
                            }
                            _ =>
                                Ok(())
                        }
                    }
                    Err(e) => {
                        error!("An error has occurred while receiving update {:?}", e);
                        Ok(())
                    }
                }
            });
    hyper::rt::run(fut);
}

fn recognize(speech_client: Arc<SpeechClient>, voice: Vec<u8>, lang: String) -> impl Future<Item=String, Error=()> {
    let encoded_voice = encode(&voice);
    speech_client.recognize(RecognizeRequest {
        config: RecognitionConfig {
            encoding: AudioEncoding::OggOpus,
            sample_rate_hertz: 16000,
            language_code: lang,
        },
        audio: RecognitionAudio { content: encoded_voice },
    }).then(|recognition_result| {
        match recognition_result {
            Ok(recognition_result) =>
                Ok(recognition_result
                    .results
                    .and_then(|results| results
                        .into_iter()
                        .nth(0)
                        .and_then(|x| x.alternatives.into_iter().nth(0).map(|x| x.transcript)))
                    .unwrap_or(String::from("Got empty result from speech api"))),
            Err(e) => {
                let error_msg = format!("Error response from google {:?}", e);
                error!("{}", error_msg);
                Ok(error_msg)
            }
        }
    })
}
