# 参考：
# https://alexbrand.dev/post/how-to-package-rust-applications-into-minimal-docker-containers/
# https://kerkour.com/rust-small-docker-image/#from-scratch

FROM rust:1.56.1 AS build
WORKDIR /usr/src

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev libssl-dev
RUN update-ca-certificates

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new juejing_actions
WORKDIR /usr/src/juejing_actions
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM alpine
COPY --from=build /usr/local/cargo/bin/juejing_actions .
USER 1000
# CMD ["./juejing_actions"]
