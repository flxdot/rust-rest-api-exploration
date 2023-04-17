# BUILDER ############################
# The builder builds the project in two steps:
#
# 1. The dependencies are build
# 2. The actual project is build
#
# This allows the layer of the dependencies to be cached
FROM rust:1-alpine as builder

# use rusts sparse-registry to fetch package info faster
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# alpine requires some additional dependencies
RUN apk add musl-dev

# Step 1: create a dummy build for the dependencies
WORKDIR /usr/src
RUN cargo new project
WORKDIR /usr/src/project
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Step 2: Build the actual project
COPY src/ ./src
# Touch main.rs to prevent cached release build
RUN touch ./src/main.rs
RUN cargo build --release

# RUNTIME ###########################
FROM alpine:3 as runtime

EXPOSE 8000

COPY --from=builder /usr/src/project/target/release/rust-rest-api-exploration /usr/local/bin/rust-rest-api-exploration

CMD ["/usr/local/bin/rust-rest-api-exploration"]