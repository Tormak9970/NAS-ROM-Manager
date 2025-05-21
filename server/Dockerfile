FROM rust:bookworm as build

# create a new empty shell project
RUN USER=root cargo new --bin nrm_server
WORKDIR /nrm_server

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/nrm_server*
RUN cargo build --release

# our final base
FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl

# copy the build artifact from the build stage
COPY --from=build /nrm_server/target/release/nrm-server .

RUN apt-get install curl ca-certificates --no-install-recommends -yqq

RUN mkdir -p /config/NRM
COPY ./parsers /config/NRM/parsers

EXPOSE 1500

# set the startup command to run your binary
CMD ["./nrm-server"]


