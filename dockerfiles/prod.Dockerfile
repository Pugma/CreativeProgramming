FROM rust:1.78-alpine3.19

WORKDIR /app

COPY . .

RUN cargo build --release

ENTRYPOINT [ "/app/target/release/server" ]