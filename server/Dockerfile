# Builder Stage
FROM rust:1.73-slim-buster as builder
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y build-essential libsqlite3-dev
COPY . .
RUN cargo build --release

# Runner Stage
FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/server /server
RUN apt-get update && apt-get install -y libsqlite3-0
CMD ["/server"]
