# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.80
ARG APP_NAME=backend

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git npm

RUN npm install -g pnpm

ADD Cargo.toml Cargo.lock ./

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > dummy.rs
RUN sed -i 's/src\/main.rs/dummy.rs/g' Cargo.toml
RUN cargo build  --locked --release
RUN sed -i 's/dummy.rs/src\/main.rs/g' Cargo.toml

COPY pnpm-lock.yaml package.json ./

COPY tailwind.config.mjs ./

COPY style/favicon.ico ./style/favicon.ico

ADD templates/ ./templates/
ADD style/ ./style/


RUN pnpm i

RUN mkdir assets
RUN mv node_modules/htmx.org/dist/htmx.min.js ./assets/
RUN mv node_modules/uikit/dist/js/uikit.min.js ./assets/
RUN mv node_modules/uikit/dist/js/uikit-icons.min.js ./assets/
RUN mv node_modules/htmx.org/dist/ext/ws.js ./assets/

RUN pnpx tailwindcss build -i ./style/main.css -o ./assets/main.css --minify

ADD src ./src

RUN  cargo build --locked --release && \
  cp ./target/release/$APP_NAME /bin/server && \
  cp -r ./assets /bin/assets && \
  cp -r ./style/ /bin/style

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
FROM alpine:3.18 AS final


# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/
COPY --from=build /bin/assets/* /assets/
COPY --from=build /bin/style/* /style/

# Expose the port that the application listens on.
EXPOSE 8000

# What the container should run when it is started.
CMD ["/bin/server"]
