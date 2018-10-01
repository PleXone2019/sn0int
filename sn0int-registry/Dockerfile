FROM rust
RUN rustup install nightly
WORKDIR /usr/src/sn0int-registry
COPY . .
RUN cargo +nightly build --release --verbose
RUN strip target/release/sn0int-registry

FROM debian
COPY --from=0 /usr/src/sn0int-registry/target/release/sn0int-registry /usr/local/bin/sn0int-registry
ENV ROCKET_ENV=prod \
    ROCKET_ADDRESS=0.0.0.0 \
    ROCKET_PORT=8000
USER www-data
ENTRYPOINT ["sn0int-registry"]