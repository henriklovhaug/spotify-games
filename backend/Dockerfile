FROM rust:alpine as builder

RUN apk add --no-cache musl-dev
RUN apk add libressl-dev

RUN cargo new backend

WORKDIR /backend


# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# 4. Build for release.
RUN cargo build --release

RUN apk add --no-cache ca-certificates

# 5. Copy the statically-linked binary into a scratch container.
FROM scratch

COPY --from=builder /backend/target/release/backend /backend
CMD [ "./backend" ]
