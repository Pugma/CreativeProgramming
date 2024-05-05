FROM rust:1.78-alpine3.19

WORKDIR /app

COPY /server .

RUN apk add musl-dev && cargo build --release

ENTRYPOINT [ "/app/target/release/server" ]