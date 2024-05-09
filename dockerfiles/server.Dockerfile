# API生成
FROM node:22.1-alpine3.19 AS api-generator

WORKDIR /app

COPY . .

WORKDIR /app/client

RUN apk add openjdk11-jre-headless &&\
    npm ci && npm run api-server


# server のビルド & 最終的な配信
FROM rust:1.78 AS server

WORKDIR /server

COPY --from=api-generator /app/server .

RUN cargo build --release

ENTRYPOINT [ "./server/target/release/app" ]