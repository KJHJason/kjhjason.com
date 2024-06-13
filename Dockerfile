# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:latest

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo install --path . # note: cargo install will automatically build the project with the --release flag

# Run the web service on container startup.
CMD ["blog-api"]
