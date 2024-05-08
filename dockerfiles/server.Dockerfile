# API生成
FROM docker:26.1.1-alpine3.19 AS api-generator

WORKDIR /app

COPY ./docs ./docs
COPY ./server ./server

RUN docker run --rm -v "${PWD}:/local" -u $(id -u) openapitools/openapi-generator-cli generate \
    -i /local/docs/openapi.yaml \
    -g rust \
    -o /local/server/apis


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