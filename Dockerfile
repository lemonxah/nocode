FROM rust:1.49.0 as builder
RUN rustup default nightly-2021-01-01
COPY Cargo.toml .
COPY Rocket.toml .
RUN cargo fetch # this should download dependencies
COPY src/ ./src/

RUN ["cargo", "build", "-Z", "unstable-options", "--out-dir", "output"]

FROM archlinux
RUN pacman -Suy --noconfirm
RUN pacman -S openssl --noconfirm
COPY --from=builder \
    output/rules \
    /
COPY --from=builder \
    /Rocket.toml \
    /

CMD /rules
