FROM node:latest AS interface

WORKDIR /usr/src/app

COPY frontend/package.json frontend/yarn.lock ./

RUN yarn

COPY ./frontend .

RUN yarn build:prod

FROM rust:latest as build

RUN apt-get update -y && \
    apt-get upgrade -y

WORKDIR /usr/src/my_blog
COPY ./backend .

RUN rustup toolchain install nightly && \
    rustup default nightly

RUN cargo update
RUN cargo build --release

FROM alpine:latest as the_end

ENV PATH_ZIP="my_blog"

RUN apk update && apk add zip \
    libtool

WORKDIR /my_blog

## execs
COPY --from=build /usr/src/my_blog/target/release/my_blog .
COPY --from=interface /usr/src/app/build ./public/

# assets
COPY myblog.service .
COPY backend/.env env
RUN mkdir images/

RUN zip -r ${PATH_ZIP}.zip /my_blog

VOLUME /dist/

CMD cp ${PATH_ZIP}.zip /dist/