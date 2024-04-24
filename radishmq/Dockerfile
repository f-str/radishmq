# Stage 1: Build the application
FROM rust:bookworm as builder

WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the build environment
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the build environment
COPY src ./src

# Build the application
RUN cargo build --release


# Stage 2: Create a lightweight production image
FROM debian:bookworm-slim

RUN apt-get update && apt-get upgrade -y && apt-get install -y openssl

RUN adduser --system --group radishmq

WORKDIR /usr/src/app

# Copy the built binary from the previous stage
COPY --from=builder /usr/src/app/target/release/radishmq ./

# Set the environment variable for logging
ENV LOGGING_ENABLED true

# Expose the desired port
EXPOSE 8080

# Change the user to the non-root user
USER radishmq

# Start the application
CMD ["./radishmq", "--address", "0.0.0.0"]

