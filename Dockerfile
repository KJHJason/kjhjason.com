# nodejs
FROM node:latest as node

WORKDIR /app
COPY . .

# install npm packages like tailwindcss
RUN npm install
RUN npm run build

FROM rust:latest

# NOTE: env var must be the same as the workdir used in the node stage
ENV APP_DIR /app

# Copy local code to the container image.
WORKDIR $APP_DIR
COPY --from=node $APP_DIR .

# Install production dependencies and build a release artifact.
# Note: cargo install will automatically build the project with the --release flag
RUN cargo install --path . 

# Run the web service on container startup.
CMD ["kjhjason-blog"]
