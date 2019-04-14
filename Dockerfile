FROM rust:latest as build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /root/source

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/docker-rs*

COPY ./ /root/source

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN mkdir -p /build-out


RUN cp -r target/x86_64-unknown-linux-musl/release /build-out/

FROM alpine:latest

RUN addgroup -g 1000 myapp
RUN adduser -D -s /bin/sh -u 1000 -G myapp myapp

COPY --from=build /build-out/release/docker-rs /app/

WORKDIR /app/

EXPOSE 3000 3000

CMD ["./docker-rs"]
