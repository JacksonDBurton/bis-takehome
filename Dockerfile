#Use the latest Rust stable release as base
FROM rust:1.80.0

# Switch Working dir, will create if not already present
WORKDIR /app

# Install sys dependencies for linking (May not be needed for me)
RUN apt-get update && apt-get install lld clang -y

# Copy all files from our environment to our Docker image
COPY . .

# Building Binary
RUN cargo build --release

ENTRYPOINT ["./target/release/bis-api"]
