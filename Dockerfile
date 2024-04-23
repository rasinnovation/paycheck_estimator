# Use the official Rust Docker image
FROM rust:alpine

# Create a working directory
WORKDIR /usr/src/app

# Copy your source code into the Docker container
COPY . .

# Install clippy
RUN rustup component add clippy

# Build your application to cache dependencies
RUN cargo build --release

# Leave a default command that does nothing, because the script will
# take care of running the necessary commands
CMD ["sleep", "infinity"]

