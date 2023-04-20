FROM rust:latest AS  builder

WORKDIR /usr/src

RUN USER=root cargo new chat_app

WORKDIR /usr/src/chat_app

COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build --release \
    && rm src/*.rs

COPY . ./

RUN rm ./target/release/deps/chat_app* \
    && cargo build --release

FROM debian:buster-slim
RUN apt-get update \
    && apt-get -y install libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/chat_app/target/release/chat_app /bin/

EXPOSE 8080

CMD ["/bin/chat_app"]

