FROM rust:1.57 as build

COPY ./ /usr/src
WORKDIR /usr/src

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends musl-tools

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=build /usr/src/target/x86_64-unknown-linux-musl/release/server /bin/server
ENV RUST_LOG=info

EXPOSE 8080

ENTRYPOINT [ "/bin/server" ]