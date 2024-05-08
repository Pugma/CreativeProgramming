# API生成
FROM node:22.1-alpine3.19 AS api-generator

WORKDIR /app

COPY . .

WORKDIR /app/client

RUN apk --update add install -y default-jre &&\
    npm ci && npm run api-server


# server のビルド & 最終的な配信
FROM rust:1.78-alpine3.19 AS server

WORKDIR /server

COPY --from=api-generator /app/server .

RUN apk add musl-dev && cargo build --release

ENTRYPOINT [ "./server/target/release/app" ]