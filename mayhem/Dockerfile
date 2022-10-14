# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest AS builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

# Install Submodules
RUN git submodule update --init --recursive
RUN cd fuzz && cargo fuzz build

# Package Stage
FROM rustlang/rust:nightly
COPY --from=builder /src/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_* /fuzz/