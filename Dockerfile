# nodejs
FROM node:latest as node

WORKDIR /app
COPY . .

# install npm packages like tailwindcss
RUN npm install
RUN npm run build

# Minify js files in the static folder
RUN npm install -g uglify-js
RUN find ./static/js -name "*.js" -exec sh -c 'uglifyjs "${0}" -c -m -o "${0%.*}.js"' {} \;

FROM rust:latest

# NOTE: env var must be the same as the workdir used in the node stage
ENV APP_DIR /app

# Copy local code to the container image.
WORKDIR $APP_DIR
COPY --from=node $APP_DIR .

# Enable async file io for better performance.
# Note: io-uring is only available on linux.
# Hence, it's disabled by default since I'm running on Windows
RUN cargo add actix-files --features experimental-io-uring

# Install production dependencies and build a release artifact.
# Note: cargo install will automatically build the project with the --release flag
RUN cargo install --path . 

# Remove artifacts after compilation to save storage space
RUN cargo clean

EXPOSE 8080

# Run the web service on container startup.
CMD ["kjhjason-blog"]
