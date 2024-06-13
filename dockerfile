FROM node:latest AS interface

WORKDIR /usr/src/app

COPY frontend/package.json frontend/yarn.lock ./

RUN yarn

COPY ./frontend .

RUN yarn build:prod

FROM rust:latest as build

ENV PATH_ZIP="my_blog"

RUN apt-get update -y && \
    apt-get upgrade -y && \
    apt-get install zip

RUN rustup toolchain install nightly && \
    rustup default nightly

RUN export LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu

WORKDIR /usr/src/my_blog
COPY . .

RUN cargo build --release

FROM alpine:latest as the_end

RUN apk update && apk add zip \
    libtool

WORKDIR /my_blog

## execs
COPY --from=build /usr/src/my_blog/target/release/main .
COPY --from=interface /usr/src/app/build ./public/

# assets
COPY ./config.toml .
COPY ./assets/ assets
COPY myblog.service .
RUN mkdir images/

RUN zip -r ${PATH_ZIP}.zip /my_blog

VOLUME /dist/

