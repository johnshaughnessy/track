# Changes to this file will cause the workflow to rebuild the builder image
FROM rust:1.73-slim-buster as builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
COPY . .
WORKDIR /track
RUN trunk build web-client/index.html # Build once so that dependencies are cached
RUN rm -rf web-client/dist # The built files are not needed in the final image
