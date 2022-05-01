FROM rust:1.60 as build

RUN USER=root cargo new --bin boop_bot
WORKDIR /boop_bot

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/boop_bot*
RUN cargo build --release

# our final base
FROM rust:1.60-slim-buster

# copy the build artifact from the build stage
COPY --from=build /boop_bot/target/release/boop_bot .

# set the startup command to run your binary
CMD ["./boop_bot"]