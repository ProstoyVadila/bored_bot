FROM rust:1.72.0-alpine3.17 as builder
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
# if needed, add additional dependencies here
RUN apk add --no-cache musl-dev pkgconfig openssl-dev
# set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app
# do a release build
RUN cargo build --release
RUN strip target/release/mr_bored_bot

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.17
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc
# copy the binary into the final image
COPY --from=builder /app/target/release/mr_bored_bot .
COPY --from=builder /app/.env .
# set the binary as entrypoint
ENTRYPOINT ["/mr_bored_bot"]
