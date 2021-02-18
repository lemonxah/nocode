FROM node:15.8.0-alpine as node_builder
ARG VUE_APP_API_URL

ENV VUE_APP_API_URL=${VUE_APP_API_URL}

# update and install dependency
RUN apk update && apk upgrade
RUN apk add git

# copy the app, note .dockerignore
COPY . .
RUN yarn
# build necessary, even if no static files are needed,
# since it builds the server as well
RUN yarn build

FROM rust:1.49.0 as builder
RUN rustup default nightly-2021-01-01
COPY Cargo.toml .
COPY Rocket.toml .
COPY dummy.rs .
RUN cargo fetch # this should download dependencies
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY src/ ./src/

RUN ["cargo", "build", "--release", "-Z", "unstable-options", "--out-dir", "output"]

FROM ubuntu
RUN apt-get update  && apt-get upgrade -y &&  apt-get install openssl -y

COPY --from=node_builder \
    dist \
    /dist

COPY --from=builder \
    output/rules \
    /
COPY --from=builder \
    /Rocket.toml \
    /

CMD /rules
