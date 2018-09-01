FROM rust:1.28.0

WORKDIR /usr/src/voices-to-text
COPY . .
RUN apt-get update
RUN apt-get install -y libav-tools
RUN cargo install --path .

CMD ["voices-to-text"]