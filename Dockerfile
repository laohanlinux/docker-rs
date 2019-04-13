FROM rust:1.31 as build

ENV RUSTUP_DIST_SERVER http://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT http://mirrors.ustc.edu.cn/rust-static/rustup

COPY ./ ./

RUN cargo build --release

RUN mkdir -p /build-out

RUN cp -r target/release /build-out/

FROM ubuntu@sha256:5f4bdc3467537cbbe563e80db2c3ec95d548a9145d64453b06939c4592d67b6d

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /build-out/ /app/

EXPOSE 3000 3000

CMD /app/release/docker-rs
