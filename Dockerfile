FROM rust:alpine as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/minimal-http-responder /usr/local/bin/minimal-http-responder
ENTRYPOINT ["minimal-http-responder"]
