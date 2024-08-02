ARG RUST_VERSION=1.80.0
FROM rust:${RUST_VERSION}-slim-bookworm AS rust_build

WORKDIR /app
COPY . .

# Enable async file io for better performance.
# Note: io-uring is only available on linux.
# Hence, it's disabled by default since I'm running on Windows
RUN cargo add actix-files --features experimental-io-uring

# Build the project
RUN cargo build --release

# ---------------------------------------------------------------------------

FROM node:lts-bookworm-slim AS node_build

WORKDIR /app
COPY . .

# install npm packages like tailwindcss
RUN npm install

# Run the build script to 
# generate a minified TailwindCSS file
RUN npm run build
RUN rm ./static/css/tailwind.css

# Minify js files in the static folder
RUN npm install -g uglify-js
RUN find ./static/js -name "*.js" -exec sh -c 'uglifyjs "${0}" -c -m -o "${0%.*}.js"' {} \;

# ---------------------------------------------------------------------------

FROM debian:bookworm-slim

# NOTE: env var must be the same as the workdir
# used in the node_build and rust_build stage
ENV APP_DIR /app

# Copy the compiled binary and node modules to the container image.
WORKDIR $APP_DIR
COPY --from=node_build $APP_DIR .
COPY --from=rust_build $APP_DIR/target/release/kjhjason-blog .

EXPOSE 8080

# Run the web service on container startup.
CMD [ "./kjhjason-blog" ]
