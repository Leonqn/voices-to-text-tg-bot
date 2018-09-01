FROM rust:1.28.0

WORKDIR /usr/src/voices-to-text
COPY . .
RUN sudo apt-get update
RUN sudo apt-get install libav-tools
RUN cargo install --path .

CMD ["voices-to-text"]