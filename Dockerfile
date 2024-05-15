# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.78
ARG APP_NAME=spotify_game

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git npm

RUN npm install -g pnpm

COPY pnpm-lock.yaml package.json ./

COPY tailwind.config.mjs ./

ADD templates/ ./templates/
ADD styles/ ./styles/

RUN pnpm i

RUN mkdir assets
RUN mv node_modules/htmx.org/dist/htmx.min.js ./assets/
RUN mv node_modules/uikit/dist/js/uikit.min.js ./assets/
RUN mv node_modules/uikit/dist/js/uikit-icons.min.js ./assets/
RUN mv node_modules/htmx.org/dist/ext/ws.js ./assets/

RUN pnpx tailwindcss build -i ./styles/tailwind.css -o ./assets/main.css --minify

# Awesome caching strategy
# RUN --mount=type=bind,source=src,target=src \
#   --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
#   --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
#   --mount=type=bind,source=templates,target=templates \
#   --mount=type=bind,source=styles,target=styles \
#   --mount=type=cache,target=/app/target/ \
#   --mount=type=cache,target=/usr/local/cargo/git/db \
#   --mount=type=cache,target=/usr/local/cargo/registry/ \


ADD Cargo.toml Cargo.lock ./
ADD src ./src

RUN  cargo build --locked --release && \
  cp ./target/release/$APP_NAME /bin/server && \
  cp -r ./assets /bin/assets

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/
COPY --from=build /bin/assets/* /assets/

# Expose the port that the application listens on.
EXPOSE 8000

# What the container should run when it is started.
CMD ["/bin/server"]
