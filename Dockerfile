FROM rust:1.28.0

WORKDIR /usr/src/voices-to-text
COPY . .

RUN cargo install --path .

CMD ["voices-to-text"]