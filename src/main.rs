extern crate voices_to_text;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;extern crate tokio;
extern crate hyper;
extern crate hyper_tls;
extern crate base64;
extern crate futures;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate config;


use hyper::rt::Future;
use voices_to_text::telegram::bot_client::BotClient;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::sync::Arc;
use hyper::Body;
use voices_to_text::google::speech_client::SpeechClient;
use base64::encode;
use voices_to_text::google::requests::*;
use hyper::rt::Stream;
use voices_to_text::telegram::requests::*;
use config::Config;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub bot_apikey: String,
    pub google_apikey: String,
    pub lang: String
}

fn main() {
    env_logger::init();
    let mut settings = Config::default();
    settings
        .merge(config::File::with_name("settings.json").required(false))
        .and_then(|x| x.merge(config::Environment::with_prefix("VTT")))
        .expect("can't find settings");

    let config: Settings = settings.try_into().expect("Wrong config");

    let https = HttpsConnector::new(1).expect("TLS initialization failed");
    let client = Arc::new(Client::builder().build::<_, Body>(https));
    let bot_client = Arc::new(BotClient::new(Arc::clone(&client), config.bot_apikey));
    let speech_client = Arc::new(SpeechClient::new(Arc::clone(&client), config.google_apikey));
    let lang = config.lang;
    let tg =
        BotClient::incoming_voice_messages(Arc::clone(&bot_client))
            .map_err(|err| error!("{:?}", err))
            .for_each(move |message| {
                let file_id = message.voice.unwrap().file_id.clone();
                let chat_id = message.chat.unwrap().id;
                let reply_to = message.message_id;
                let get_file = GetFile {
                    file_id
                };
                let lang = lang.clone();
                let speech_arc = Arc::clone(&speech_client);
                let bot_client_arc = Arc::clone(&bot_client);
                let fut = bot_client_arc.get_file(get_file)
                    .and_then(move |audio| {
                        let b64_audio = encode(&audio);
                        speech_arc.recognize(RecognizeRequest {
                            config: RecognitionConfig {
                                encoding: AudioEncoding::OggOpus,
                                sample_rate_hertz: 16000,
                                language_code: lang,
                            },
                            audio: RecognitionAudio { content: b64_audio },
                        }).and_then(move |recognized| {
                            bot_client_arc.send_message(SendMessage {
                                chat_id,
                                text: recognized.results.into_iter().nth(0).and_then(|x| x.alternatives.into_iter().nth(0).map(|x| x.transcript)).unwrap_or(String::from("Cant recognize")),
                                reply_to_message_id: Some(reply_to),
                            })
                        })
                    })
                    .map(|_| ())
                    .map_err(|err| error!("{:?}", err));
                hyper::rt::spawn(fut);
                Ok(())
            });

    hyper::rt::run(tg);
}
