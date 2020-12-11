FROM ekidd/rust-musl-builder:latest as build

ADD --chown=rust:rust . ./
RUN cargo build --release

FROM alpine:latest

COPY --from=build \
  /home/rust/src/target/x86_64-unknown-linux-musl/release/taskach-backend \
  /usr/local/bin/

CMD /usr/local/bin/taskach-backend