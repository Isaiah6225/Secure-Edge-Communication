ARG RUST_VERSION=1.93.1
ARG APP_NAME=server-code

FROM docker.io/library/rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

#install host build dependencies 
RUN apk add --no-cache clang lld musl-dev git

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server


FROM docker.io/library/alpine:3.18 AS final

# Create a non-privileged user (recommended best practice)
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# Drop privileges for runtime.
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/
COPY .cargo/config.toml ./.cargo/config.toml

ENV IP=0.0.0.0:7979

# Expose the port that the application listens on.
EXPOSE 7979 

# What the container should run when it is started.
CMD ["/bin/server"]
