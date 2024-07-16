# Use the official Rust image as the base image
FROM rust:latest as builder

# Install libclang and dependencies for RocksDB
RUN apt-get update && apt-get install -y clang librocksdb-dev

# Create a new directory for the project
WORKDIR /usr/src/near_block_listener

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the project in debug mode (faster)
RUN cargo build --release

# Use a smaller image for the final container
FROM ubuntu:22.04

# Install RocksDB dependencies, OpenSSL, and ca-certificates
RUN apt-get update && apt-get install -y librocksdb-dev libssl-dev ca-certificates

# Copy the build artifact from the build stage
COPY --from=builder /usr/src/near_block_listener/target/release/earthmind_client_near /usr/local/bin/near_block_listener

# Ensure that the dynamic linker can find the OpenSSL 3 libraries
RUN ldconfig

# Run the binary
CMD ["near_block_listener"]
