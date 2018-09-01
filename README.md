# voices-to-text-tg-bot
Simple bot that converts telegram voice and video note messages to text.  
This bot will reply on any voice or video note message either in private or in group with text recognized by google speech api.

Run in docker:
``` console
$ docker build -t voices-to-text .
$ docker run -d --rm -e bot_apikey='YOUR BOT API KEY' -e speech_apikey='YOUR GOOGLE SPEECH API KEY' -e lang='LANG CODE FOR GOOGLE SPEECH' voices-to-text 
```
Telegram bot api key you can get from [BotFather](https://telegram.me/botfather)\
Google speech api key you can get from [GCP console](https://console.cloud.google.com/apis/credentials)\
Language code for your language you can get [here](https://cloud.google.com/speech-to-text/docs/languages)
