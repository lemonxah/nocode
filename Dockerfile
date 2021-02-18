FROM node:15.8.0-alpine as node_builder
ARG API_URL

ENV API_URL=${API_URL}

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
RUN cargo fetch # this should download dependencies
COPY src/ ./src/

RUN ["cargo", "build", "-Z", "unstable-options", "--out-dir", "output"]

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
