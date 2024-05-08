# API生成
FROM openapitools/openapi-generator-cli AS api-generator

WORKDIR /app

COPY . .

RUN generate \
    -i /app/docs/openapi.yaml \
    -g rust \
    -o /app/server/apis


# server のビルド
FROM rust:1.78-alpine3.19 AS server

WORKDIR /server

COPY --from=api-generator /app/server .

RUN apk add musl-dev && cargo build --release

# 最終的な配信用
FROM alpine:3.19.1

WORKDIR /app

COPY --from=server /server/target/release/app app

ENTRYPOINT [ "./app" ]